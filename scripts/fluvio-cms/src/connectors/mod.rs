pub mod cli;
pub use cli::ConnectorsOpt;

use std::collections::HashMap;

use clap::ValueEnum;
use lazy_static::lazy_static;
use strum::Display;

lazy_static! {
    pub static ref INBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();
        m.insert(
            DataService::Http,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "http-source",
                hub_pkg_version: "0.1.1",
                service: DataService::Http,
                direction: DataDirection::Inbound,
            },
        );

        m.insert(
            DataService::Mqtt,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "mqtt-source",
                hub_pkg_version: "0.1.2",
                service: DataService::Mqtt,
                direction: DataDirection::Inbound,
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "kafka-source",
                hub_pkg_version: "0.1.1",
                service: DataService::Kafka,
                direction: DataDirection::Inbound,
            },
        );

        m
    };
    pub static ref OUTBOUND: HashMap<DataService, DocsLocation> = {
        let mut m = HashMap::new();

        m.insert(
            DataService::Sql,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "sql-sink",
                hub_pkg_version: "0.1.1",
                service: DataService::Sql,
                direction: DataDirection::Outbound,
            },
        );

        m.insert(
            DataService::Kafka,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "kafka-sink",
                hub_pkg_version: "0.1.1",
                service: DataService::Kafka,
                direction: DataDirection::Outbound,
            },
        );

        m.insert(
            DataService::Http,
            DocsLocation {
                hub_group: "infinyon",
                hub_pkg_name: "http-sink",
                hub_pkg_version: "0.1.0",
                service: DataService::Http,
                direction: DataDirection::Outbound,
            },
        );

        m
    };
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DocsLocation {
    hub_group: &'static str,
    hub_pkg_name: &'static str,
    hub_pkg_version: &'static str,
    service: DataService,
    direction: DataDirection,
}

/// Selector for the data protocol of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq, Copy, Hash, Display)]
#[clap(rename_all = "lowercase")]
#[strum(serialize_all = "UPPERCASE")]
pub enum DataService {
    #[default]
    All,
    #[strum(serialize = "Kafka")]
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
#[strum(serialize_all = "lowercase")]
pub enum DataDirection {
    #[default]
    All,
    #[clap(alias = "source")]
    Inbound,
    #[clap(alias = "sink")]
    Outbound,
}
