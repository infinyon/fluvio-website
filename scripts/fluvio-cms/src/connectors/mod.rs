use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use strum::Display;
use tracing::info;

use std::fs::{self, File};
use std::io::{BufReader, Read, Write};

// This is a bit verbose until reference docs are fetched from Hub
lazy_static! {
    static ref INBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();
        m.insert(
            DataService::Http,
            DocsLocation {
                _hub: "infinyon/http-source@0.1.0",
                github: "https://raw.githubusercontent.com/infinyon/http-connector/main/README.md",
                hugo_embed: "embeds/connectors-beta/inbound/http.md".into(),
                hugo_content: "content/connectors-beta/inbound/http.md".into(),
                use_remote_sync: true,
            },
        );

        m.insert(
            DataService::Mqtt,
            DocsLocation {
                _hub: "infinyon/mqtt-source@0.1.0",
                github: "https://raw.githubusercontent.com/infinyon/mqtt-connector/main/README.md",
                hugo_embed: "embeds/connectors-beta/inbound/mqtt.md".into(),
                hugo_content: "content/connectors-beta/inbound/mqtt.md".into(),
                use_remote_sync: true,
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                _hub: "infinyon/kafka-source@0.4.0",
                // This doesn't exist yet
                github: "https://raw.githubusercontent.com/infinyon/kafka-connector/main/crates/kafka-source/README.md?token=GHSAT0AAAAAAB7VGJIUUUMLFZG6IOE6HREWZASDYWA",
                hugo_embed: "embeds/connectors-beta/inbound/kafka.md".into(),
                hugo_content: "content/connectors-beta/inbound/kafka.md".into(),
                use_remote_sync: false,
            },
        );

        m

    };

    static ref OUTBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();

        m.insert(
            DataService::Sql,
            DocsLocation {
                _hub: "infinyon/sql-sink@0.1.0",
                github: "https://raw.githubusercontent.com/infinyon/sql-connector/main/README.md?token=GHSAT0AAAAAAB7VGJIVGR6VOC3XANRNZTWCZASDXMA",
                hugo_embed: "embeds/connectors-beta/outbound/sql.md".into(),
                hugo_content: "content/connectors-beta/outbound/sql.md".into(),
                use_remote_sync: false,
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                // This doesn't exist yet
                _hub: "",
                // This doesn't exist yet
                github: "https://raw.githubusercontent.com/infinyon/kafka-connector/main/crates/kafka-sink/README.md?token=GHSAT0AAAAAAB7VGJIUUUMLFZG6IOE6HREWZASDYWA",
                hugo_embed: "embeds/connectors-beta/outbound/kafka.md".into(),
                hugo_content: "content/connectors-beta/outbound/kafka.md".into(),
                use_remote_sync: false,
            },
        );

        m
    };

}

//
struct DocsLocation {
    // Not yet implemented, but assuming the README will be available from the connector package
    _hub: &'static str,
    // This is just a short-term workaround
    github: &'static str,

    // This is where Hugo templates expect README.md for embedding
    hugo_embed: PathBuf,
    hugo_content: PathBuf,

    // This is temporary, for giving error message
    use_remote_sync: bool,
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

        let readme = if let Some(f) = &self.localfile {
            let f = File::open(f)?;
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            // Read file into vector.
            reader.read_to_end(&mut buffer)?;
            buffer.into()
        } else if docs.use_remote_sync {
            self.download_connector_ref(docs)?
        } else {
            return Err(anyhow!("Syncing docs remotely not yet supported. Use `--file` to point at reference doc for connector on local disk"));
        };

        self.write_ref_to_disk(docs, direction, protocol, readme)
    }

    fn download_connector_ref(&self, docs: &DocsLocation) -> Result<Bytes> {
        info!("Downloading refs");

        fluvio_future::task::run_block_on(async {
            let resp = reqwest::get(docs.github)
                .await
                .map_err(|e| anyhow!(e.to_string()))?;

            if resp.status().is_success() {
                resp.bytes().await.map_err(|e| anyhow!(e.to_string()))
            } else {
                Err(anyhow!("Unable to download docs at {}", docs.github))
            }
        })
    }

    fn write_ref_to_disk(
        &self,
        docs: &DocsLocation,
        direction: DataDirection,
        protocol: DataService,
        readme: Bytes,
    ) -> Result<()> {
        // Write the embeddable readme
        let mut embed_file = fs::OpenOptions::new()
            .create(true) // To create a new file
            .write(true)
            .open(docs.hugo_embed.as_path())
            .unwrap();

        info!("Write the README file to {}", docs.hugo_embed.display());
        embed_file.write_all(&readme).unwrap();

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

{{{{% inline-embed file="embeds/connectors-beta/{direction}/{connector_file_name}.md" %}}}}"
"#,
            direction = direction.to_string().to_ascii_lowercase(),
            connector_name_title = protocol.to_string().to_ascii_uppercase(),
            connector_file_name = protocol.to_string().to_ascii_lowercase(),
        )?;

        Ok(())
    }
}
