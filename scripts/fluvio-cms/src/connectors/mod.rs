use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use strum::Display;
use tracing::info;
use xshell::{cmd, Shell};

// This is a bit verbose until reference docs are fetched from Hub
lazy_static! {
    static ref INBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();
        m.insert(
            DataService::Http,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "http-source",
                hub_pkg_version: "0.1.1",
                hugo_embed: "embeds/connectors-beta/inbound/http.md".into(),
                hugo_content: "content/connectors-beta/inbound/http.md".into(),
            },
        );

        m.insert(
            DataService::Mqtt,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "mqtt-source",
                hub_pkg_version: "0.1.2",
                hugo_embed: "embeds/connectors-beta/inbound/mqtt.md".into(),
                hugo_content: "content/connectors-beta/inbound/mqtt.md".into(),
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "kafka-source",
                hub_pkg_version: "0.1.1",
                hugo_embed: "embeds/connectors-beta/inbound/kafka.md".into(),
                hugo_content: "content/connectors-beta/inbound/kafka.md".into(),
            },
        );

        m
    };
    static ref OUTBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();

        m.insert(
            DataService::Sql,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "sql-sink",
                hub_pkg_version: "0.1.1",
                hugo_embed: "embeds/connectors-beta/outbound/sql.md".into(),
                hugo_content: "content/connectors-beta/outbound/sql.md".into(),
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "kafka-sink",
                hub_pkg_version: "0.1.1",
                hugo_embed: "embeds/connectors-beta/outbound/kafka.md".into(),
                hugo_content: "content/connectors-beta/outbound/kafka.md".into(),
            },
        );

        m
    };
}

//
struct DocsLocation {
    hub_group: &'static str,
    hub_pkg_name: &'static str,
    hub_pkg_version: &'static str,
    // This is where Hugo templates expect README.md for embedding
    hugo_embed: PathBuf,
    hugo_content: PathBuf,
}

/// Selector for the data protocol of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq, Copy, Hash, Display)]
#[clap(rename_all = "lowercase")]
pub enum DataService {
    #[default]
    All,
    Kafka,
    Http,
    Sql,
    Mqtt,
    //Salesforce,
    //Amplitude,
}

/// Selector for the data direction of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq, Copy, Hash, Display)]
#[clap(rename_all = "lowercase")]
pub enum DataDirection {
    #[default]
    All,
    #[clap(alias = "source")]
    Inbound,
    #[clap(alias = "sink")]
    Outbound,
}

/// Update the reference docs for a connector. By default, update all
#[derive(Clone, Parser, Debug)]
pub struct ConnectorsOpt {
    //#[clap(value_enum)]
    //doc: DocScheme,
    /// Service or protocol used by connector to join data with Fluvio cluster
    #[clap(long)]
    protocol: Option<DataService>,
    /// Direction data flows through connector wrt Fluvio cluster. Aliases: source, sink
    #[clap(long)]
    direction: Option<DataDirection>,
    /// Path to README on the local filesystem,
    #[clap(long = "file", requires_all = ["protocol", "direction"])]
    localfile: Option<PathBuf>,
}

impl ConnectorsOpt {
    pub fn run(&self) -> Result<()> {
        let direction = self.direction.unwrap_or(DataDirection::All);
        let protocol = self.protocol.unwrap_or(DataService::All);

        if protocol == DataService::All {
            return Err(anyhow!(
                "Not supported yet. Please choose individual connectors"
            ));
        }

        let docs = match direction {
            DataDirection::Inbound => INBOUND.get(&protocol).unwrap(),
            DataDirection::Outbound => OUTBOUND.get(&protocol).unwrap(),
            _ => {
                return Err(anyhow!(
                    "Not supported yet. Please choose specific direction"
                ))
            }
        };

        let _readme = if let Some(f) = &self.localfile {
            let f = File::open(f)?;
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            // Read file into vector.
            reader.read_to_end(&mut buffer)?;
            //buffer.into()
        } else {
            //self.download_connector_ref(docs)?

            // TODO: Write into temp directory

            let sh = Shell::new()?;

            let hub_group = docs.hub_group;
            let hub_pkg_name = docs.hub_pkg_name;
            let hub_pkg_version = docs.hub_pkg_version;
            cmd!(
                sh,
                "fluvio hub connector download {hub_group}/{hub_pkg_name}@{hub_pkg_version} --remote https://hub-dev.infinyon.cloud"
            )
            .run()?;
            cmd!(
                sh,
                "tar xvf {hub_group}-{hub_pkg_name}-{hub_pkg_version}.ipkg"
            )
            .run()?;
            cmd!(sh, "tar xvf manifest.tar.gz").run()?;
        };

        self.write_ref_to_disk(docs, direction, protocol, "README.md".into())?;
        Ok(())
    }

    //fn download_connector_ref(&self, docs: &DocsLocation) -> Result<Bytes> {
    //    info!("Downloading refs");

    //    fluvio_future::task::run_block_on(async {
    //        let resp = reqwest::get(docs.github)
    //            .await
    //            .map_err(|e| anyhow!(e.to_string()))?;

    //        if resp.status().is_success() {
    //            resp.bytes().await.map_err(|e| anyhow!(e.to_string()))
    //        } else {
    //            Err(anyhow!("Unable to download docs at {}", docs.github))
    //        }
    //    })
    //}

    fn write_ref_to_disk(
        &self,
        docs: &DocsLocation,
        direction: DataDirection,
        protocol: DataService,
        readme: PathBuf,
    ) -> Result<()> {
        // Write the embeddable readme
        //let mut embed_file = fs::OpenOptions::new()
        //    .create(true) // To create a new file
        //    .write(true)
        //    .open(docs.hugo_embed.as_path())
        //    .unwrap();

        let sh = Shell::new()?;
        info!("Write the README file to {}", docs.hugo_embed.display());

        let embed_location = docs.hugo_embed.to_path_buf();

        cmd!(sh, "mv {readme} {embed_location}").run()?;
        //embed_file.write_all(&readme).unwrap();

        // Write the content file template
        info!(
            "Write the content template to {}",
            docs.hugo_content.display()
        );

        let mut content_file = fs::OpenOptions::new()
            .create(true) // To create a new file
            .write(true)
            .open(docs.hugo_content.as_path())?;

        write!(
            &mut content_file,
            r#"---
menu: {connector_name_title} 
---

{{{{% inline-embed file="embeds/connectors-beta/{direction}/{connector_file_name}.md" %}}}}
"#,
            direction = direction.to_string().to_case(Case::Lower),
            connector_name_title = protocol.to_string().to_case(Case::Title),
            connector_file_name = protocol.to_string().to_case(Case::Lower),
        )?;

        Ok(())
    }
}
