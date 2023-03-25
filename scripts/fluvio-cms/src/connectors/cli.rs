use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use convert_case::{Case, Casing};
use tracing::info;
use xshell::{cmd, Shell};

use super::{DataDirection, DataService, DocsLocation, INBOUND, OUTBOUND};

/// Update the reference docs for a connector. By default, update all
#[derive(Clone, Parser, Debug)]
pub struct ConnectorsOpt {
    //#[clap(value_enum)]
    //doc: DocScheme,
    /// Service or protocol used by connector to join data with Fluvio cluster
    #[clap(long)]
    service: Option<DataService>,
    /// Direction data flows through connector wrt Fluvio cluster. Aliases: source, sink
    #[clap(long)]
    direction: Option<DataDirection>,
    /// Path to README on the local filesystem,
    #[clap(long = "file", requires_all = ["service", "direction"])]
    localfile: Option<PathBuf>,
    /// Use the production instance of Hub
    #[clap(long, action)]
    prod: bool,
    /// Display shell command output
    #[clap(long, action)]
    verbose: bool,
}

impl ConnectorsOpt {
    pub fn run(&self) -> Result<()> {
        let direction = self.direction.unwrap_or(DataDirection::All);
        let protocol = self.service.unwrap_or(DataService::All);
        let mut connectors = Vec::new();

        match (direction, protocol) {
            // All the connectors
            (DataDirection::All, DataService::All) => {
                info!("Update all the connectors");
                connectors.extend(INBOUND.values());
                connectors.extend(OUTBOUND.values());
            }
            // All the inbound
            (DataDirection::Inbound, DataService::All) => {
                info!("Update all the inbound connectors");
                connectors.extend(INBOUND.values());
            }
            // All the outbound
            (DataDirection::Outbound, DataService::All) => {
                info!("Update all the outbound connectors");
                connectors.extend(OUTBOUND.values());
            }
            // All of one type
            (DataDirection::All, proto) => {
                info!("Update all the {proto:?} connectors");
                if let Some(inbound) = INBOUND.get(&proto) {
                    connectors.push(inbound);
                }
                if let Some(outbound) = OUTBOUND.get(&proto) {
                    connectors.push(outbound);
                }
            }
            // Just one
            (DataDirection::Inbound, proto) => {
                info!("Update inbound {proto:?} connector");
                if let Some(inbound) = INBOUND.get(&proto) {
                    connectors.push(inbound);
                }
            }
            (DataDirection::Outbound, proto) => {
                info!("Update outbound {proto:?} connector");
                if let Some(outbound) = OUTBOUND.get(&proto) {
                    connectors.push(outbound);
                }
            }
        }

        for connector in connectors {
            let sh = Shell::new()?;
            let temp_dir = sh.create_temp_dir()?;
            if let Some(f) = &self.localfile {
                let f = File::open(f)?;
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();

                // Read file into vector.
                reader.read_to_end(&mut buffer)?;
            } else {
                sh.change_dir(temp_dir.path());

                let hub_group = connector.hub_group;
                let hub_pkg_name = connector.hub_pkg_name;
                let hub_pkg_version = connector.hub_pkg_version;

                let hub_remote = if self.prod {
                    Vec::new()
                } else {
                    vec!["--remote", "https://hub-dev.infinyon.cloud"]
                };

                let mut cmd =
                    cmd!(sh,
                    "fluvio hub connector download {hub_group}/{hub_pkg_name}@{hub_pkg_version}"
                );
                cmd.set_ignore_stdout(!self.verbose);
                cmd.set_ignore_stderr(!self.verbose);
                cmd.args(hub_remote).run()?;

                let mut cmd = cmd!(
                    sh,
                    "tar xvf {hub_group}-{hub_pkg_name}-{hub_pkg_version}.ipkg"
                );
                cmd.set_ignore_stdout(!self.verbose);
                cmd.set_ignore_stderr(!self.verbose);
                cmd.run()?;

                let mut cmd = cmd!(sh, "tar xvf manifest.tar.gz");
                cmd.set_ignore_stdout(!self.verbose);
                cmd.set_ignore_stderr(!self.verbose);
                cmd.run()?;
            };

            let readme = temp_dir.path().join("README.md");

            self.write_ref_to_disk(connector, readme)?;
        }
        Ok(())
    }

    fn write_ref_to_disk(&self, connector: &DocsLocation, readme: PathBuf) -> Result<()> {
        let embed_connector_readme = format!(
            "embeds/connectors/{}/{}.md",
            connector.direction,
            connector.service.to_string().to_case(Case::Lower)
        );
        let connector_hugo_template = format!(
            "content/connectors/{}/{}.md",
            connector.direction,
            connector.service.to_string().to_case(Case::Lower)
        );

        let sh = Shell::new()?;

        info!("Write the README file to {embed_connector_readme}");

        sh.copy_file(readme, &embed_connector_readme)?;

        // Write the content file template
        info!("Write the content template to {connector_hugo_template}");

        let mut content_file = fs::OpenOptions::new()
            .create(true) // To create a new file
            .write(true)
            .truncate(true)
            .open(connector_hugo_template)?;

        write!(
            &mut content_file,
            r#"---
menu: {connector_name_title} 
---

{{{{% inline-embed file="{embed}" %}}}}"#,
            connector_name_title = connector.service,
            embed = embed_connector_readme
        )?;

        Ok(())
    }
}
