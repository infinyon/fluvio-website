pub mod cli;
pub use cli::ConnectorsOpt;

use anyhow::Result;
use clap::ValueEnum;
use fluvio_hub_util::PackageMeta;
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::debug;
use xshell::{cmd, Shell};


const INFINYON_GROUP: [&str; 2] = ["infinyon", "infinyon-labs"];

#[derive(Debug, Clone, Serialize, Deserialize, strum::Display, PartialEq, Eq)]
#[serde(tag = "type", content = "version")]
pub enum OfficialConnector {
    // http
    #[serde(alias = "http-source")]
    #[strum(serialize = "http-source")]
    HttpInbound(String),
    #[serde(alias = "http-sink")]
    #[strum(serialize = "http-sink")]
    HttpOutbound(String),
    // kafka
    #[serde(alias = "kafka-source")]
    #[strum(serialize = "kafka-source")]
    KafkaInbound(String),
    #[serde(alias = "kafka-sink")]
    #[strum(serialize = "kafka-sink")]
    KafkaOutbound(String),
    // mqtt
    #[serde(alias = "mqtt-source")]
    #[strum(serialize = "mqtt-source")]
    MqttInbound(String),
    // sql
    #[serde(alias = "sql-sink")]
    #[strum(serialize = "sql-sink")]
    SqlOutbound(String),
    // duckdb
    #[serde(alias = "duckdb-sink")]
    #[strum(serialize = "duckdb-sink")]
    DuckdbOutbound(String),
    #[serde(alias = "graphite-sink")]
    #[strum(serialize = "graphite-sink")]
    GraphiteOutbound(String),
}

impl OfficialConnector {
    /// Collect list of public connectors, and their latest metadata
    pub fn get_all_public(prod: bool) -> Result<Vec<ConnectorInfo>> {
        let sh = Shell::new()?;
        let cmd = cmd!(sh, "fluvio hub connector list -O json");

        let hub_remote = if !prod {
            Some(vec!["--remote", "https://hub-dev.infinyon.cloud"])
        } else {
            None
        };

        // This will fail if you're not logged into the correct cloud (dev vs prod)
        // If this happens, run `fluvio cloud login`, and use `--prod` if needed
        let latest_connectors_raw: Vec<PackageMeta> = if let Some(hub_dev_remote) = hub_remote {
            serde_json::from_str(cmd.args(hub_dev_remote).read()?.as_str())
                .expect("Error parsing output - Do you need to re-login to Dev?")
        } else {
            serde_json::from_str(cmd.read()?.as_str())
                .expect("Error parsing output - Do you need to re-login to Prod?")
        };

        let mut latest_connectors = Vec::new();

        for c in latest_connectors_raw.iter() {
            if !INFINYON_GROUP.contains(&c.group.as_str()) {
                debug!("Skipping non-InfinyOn connector");
                continue;
            }

            let meta = format!(
                "{{ \"type\": \"{}\", \"version\": \"{}\" }}",
                c.name.as_str(),
                c.version.as_str()
            );

            let parse: Result<OfficialConnector, serde_json::Error> =
                serde_json::from_str(meta.as_str());

            if let Ok(supported) = parse {
                //println!("{supported}");
                latest_connectors.push(ConnectorInfo {
                    hub_group: c.group.clone(),
                    connector: supported,
                })
            } else {
                println!("Skipping unsupported connector");
                continue;
            }
        }
        Ok(latest_connectors)
    }

    /// Returns list of the official InfinyOn supported connectors, with optional filtering
    pub fn select(
        direction: Option<DataDirection>,
        service: Option<DataServiceType>,
        prod: bool,
    ) -> Result<Vec<ConnectorInfo>> {
        let all_public = Self::get_all_public(prod)?;
        let mut selected = Vec::new();

        let service_filter = service.unwrap_or_default();
        let direction_filter = direction.unwrap_or_default();

        for conn in all_public.into_iter() {
            if conn.is_direction_match(direction_filter) && conn.is_service_match(service_filter) {
                selected.push(conn)
            }
        }

        Ok(selected)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectorInfo {
    hub_group: String,
    connector: OfficialConnector,
}

impl ConnectorInfo {
    /// Returns true if the connector's type matches the `DataServiceType` regardless of data direction flow
    pub fn is_service_match(&self, service: DataServiceType) -> bool {
        match service {
            DataServiceType::All => true,
            DataServiceType::Http => matches!(
                self.connector,
                OfficialConnector::HttpInbound(_) | OfficialConnector::HttpOutbound(_)
            ),
            DataServiceType::Kafka => matches!(
                self.connector,
                OfficialConnector::KafkaInbound(_) | OfficialConnector::KafkaOutbound(_)
            ),
            DataServiceType::Sql => matches!(self.connector, OfficialConnector::SqlOutbound(_)),
            DataServiceType::Duckdb => {
                matches!(self.connector, OfficialConnector::DuckdbOutbound(_))
            }
            DataServiceType::Mqtt => matches!(self.connector, OfficialConnector::MqttInbound(_)),
            DataServiceType::Graphite => {
                matches!(self.connector, OfficialConnector::GraphiteOutbound(_))
            }
        }
    }

    /// Returns true if the connector's `DataDirection` matches regardless of service type
    pub fn is_direction_match(&self, direction: DataDirection) -> bool {
        match direction {
            DataDirection::All => true,
            DataDirection::Inbound => matches!(
                self.connector,
                OfficialConnector::HttpInbound(_)
                    | OfficialConnector::KafkaInbound(_)
                    | OfficialConnector::MqttInbound(_)
            ),
            DataDirection::Outbound => matches!(
                self.connector,
                OfficialConnector::HttpOutbound(_)
                    | OfficialConnector::KafkaOutbound(_)
                    | OfficialConnector::SqlOutbound(_)
            ),
        }
    }

    /// The Hub group name used for publishing
    pub fn get_group(&self) -> String {
        self.hub_group.clone()
    }

    /// The value used for `type` in the connector config
    pub fn get_connector_type(&self) -> String {
        self.connector.to_string()
    }

    /// The Hub package version
    pub fn get_version(&self) -> String {
        match &self.connector {
            OfficialConnector::HttpInbound(v)
            | OfficialConnector::HttpOutbound(v)
            | OfficialConnector::KafkaInbound(v)
            | OfficialConnector::KafkaOutbound(v)
            | OfficialConnector::MqttInbound(v)
            | OfficialConnector::SqlOutbound(v)
            | OfficialConnector::DuckdbOutbound(v)
            | OfficialConnector::GraphiteOutbound(v) => v.clone(),
        }
    }

    /// Plain name for the connector's service/protocol
    pub fn get_service_type(&self) -> DataServiceType {
        self.connector.clone().into()
    }

    /// Returns a `DataDirection` with respect to the flow of data through the connector
    pub fn get_direction(&self) -> DataDirection {
        self.connector.clone().into()
    }
}

/// Selector for the data protocol of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq, Copy, Hash, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum DataServiceType {
    #[default]
    All,
    #[strum(serialize = "Kafka")]
    Kafka,
    Http,
    Sql,
    Mqtt,
    #[strum(serialize = "DuckDB")]
    Duckdb,
    #[strum(serialize = "Graphite")]
    Graphite,
    //Salesforce,
    //Amplitude,
}

impl From<OfficialConnector> for DataServiceType {
    fn from(value: OfficialConnector) -> Self {
        match value {
            OfficialConnector::HttpInbound(_) | OfficialConnector::HttpOutbound(_) => Self::Http,
            OfficialConnector::KafkaInbound(_) | OfficialConnector::KafkaOutbound(_) => Self::Kafka,
            OfficialConnector::MqttInbound(_) => Self::Mqtt,
            OfficialConnector::SqlOutbound(_) => Self::Sql,
            OfficialConnector::DuckdbOutbound(_) => Self::Duckdb,
            OfficialConnector::GraphiteOutbound(_) => Self::Graphite,
        }
    }
}

/// Selector for the data direction of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq, Copy, Hash, Display)]
#[strum(serialize_all = "lowercase")]
pub enum DataDirection {
    #[default]
    All,
    #[clap(alias = "source")]
    Inbound,
    #[clap(alias = "sink")]
    Outbound,
}

impl From<OfficialConnector> for DataDirection {
    fn from(value: OfficialConnector) -> Self {
        match value {
            OfficialConnector::HttpInbound(_)
            | OfficialConnector::KafkaInbound(_)
            | OfficialConnector::MqttInbound(_) => Self::Inbound,
            OfficialConnector::HttpOutbound(_)
            | OfficialConnector::KafkaOutbound(_)
            | OfficialConnector::SqlOutbound(_)
            | OfficialConnector::DuckdbOutbound(_)
            | OfficialConnector::GraphiteOutbound(_) => Self::Outbound,
        }
    }
}
