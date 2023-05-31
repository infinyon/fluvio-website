use std::fs::{self};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use tracing::info;
use xshell::{cmd, Shell};

//use super::{Connectors, DataDirection, DataService, DocsLocation, INBOUND, OUTBOUND};
use super::{ConnectorInfo, DataDirection, DataServiceType, OfficialConnector};

const WORKING_DIR_BASE: &str = "./.tmp";

/// Update the reference docs for a connector. By default, update all
#[derive(Clone, Parser, Debug)]
pub struct ConnectorsOpt {
    //#[clap(value_enum)]
    //doc: DocScheme,
    /// Service or protocol used by connector to join data with Fluvio cluster
    #[clap(long)]
    service: Option<DataServiceType>,
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
    /// Don't clean up files created
    #[clap(long, action)]
    no_clean: bool,
}

impl ConnectorsOpt {
    pub fn run(&self) -> Result<()> {
        let connectors = OfficialConnector::select(self.direction, self.service, self.prod)?;

        let sh = Shell::new()?;
        sh.remove_path(WORKING_DIR_BASE).unwrap_or_default();

        for c in connectors {
            let working_dir =
                sh.create_dir(format!("{}/{}", WORKING_DIR_BASE, c.get_connector_type()))?;
            let _working_path_guard = sh.push_dir(working_dir.as_path());

            let hub_pkg_fqn = format!(
                "{}/{}@{}",
                c.get_group(),
                c.get_connector_type(),
                c.get_version()
            );

            let ipkg_filename = format!(
                "{}-{}-{}.ipkg",
                c.get_group(),
                c.get_connector_type(),
                c.get_version()
            );

            let hub_remote = if self.prod {
                Vec::new()
            } else {
                vec!["--remote", "https://hub-dev.infinyon.cloud"]
            };

            // Download
            let mut cmd = cmd!(sh, "fluvio hub connector download {hub_pkg_fqn}");
            cmd.set_ignore_stdout(!self.verbose);
            cmd.set_ignore_stderr(!self.verbose);
            cmd.args(hub_remote).run()?;

            // Extract Hub raw package content
            let mut cmd = cmd!(sh, "tar -xvf {ipkg_filename} manifest.tar.gz");
            cmd.set_ignore_stdout(!self.verbose);
            cmd.set_ignore_stderr(!self.verbose);
            cmd.run()?;

            // Extract package manifest
            let mut cmd = cmd!(sh, "tar -xvf manifest.tar.gz README.md");
            cmd.set_ignore_stdout(!self.verbose);
            cmd.set_ignore_stderr(!self.verbose);
            cmd.run()?;

            // Select README.md and overwrite connector's current docs
            let readme = working_dir.join("README.md");
            self.write_ref_to_disk(&c, readme)?;
        }

        if self.no_clean {
            println!("Skipping cleanup of directory: {WORKING_DIR_BASE}");
        } else {
            sh.remove_path(WORKING_DIR_BASE).unwrap_or_default();
        }

        Ok(())
    }

    fn write_ref_to_disk(&self, connector: &ConnectorInfo, readme: PathBuf) -> Result<()> {
        let embed_connector_readme = format!(
            "embeds/connectors/{}/{}.md",
            connector.get_direction(),
            connector.get_service_type().to_string().to_lowercase()
        );
        let connector_hugo_template = format!(
            "content/connectors/{}/{}.md",
            connector.get_direction(),
            connector.get_service_type().to_string().to_lowercase()
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
            connector_name_title = connector.get_service_type(),
            embed = embed_connector_readme
        )?;

        Ok(())
    }
}
