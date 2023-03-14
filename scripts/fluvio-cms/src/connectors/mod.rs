use super::DocScheme;

use clap::{Args, Parser, Subcommand, ValueEnum};
use tracing::{debug, info};

/// Selector for the data protocol of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Copy)]
//#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum DataProtocol {
    #[default]
    All,
    Kafka,
    Http,
    Sql,
}

/// Selector for the data direction of connectors
#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Copy)]
//#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum DataDirection {
    #[default]
    All,
    #[clap(alias = "source")]
    Inbound,
    #[clap(alias = "sink")]
    Outbound,
}

/// This describes a specific connector (data vector + protocol)
//#[derive(Clone, Debug, Subcommand)]
//#[clap(rename_all = "lowercase")]
//pub enum Connector {
//    //Inbound(DataProtocol),
//    //Outbound(DataProtocol),
//    All,
//    #[clap(arg_required_else_help = true)]
//    Kafka {
//        direction: DataDirection,
//    },
//    Http,
//    Sql,
//}

#[derive(Clone, Parser, Debug)]
pub struct ConnectorsOpt {
    #[clap(value_enum)]
    doc: DocScheme,
    ////#[clap(subcommand)]
    ////connector: Connector,
    //#[clap(default_missing_value = "all")]
    protocol: Option<DataProtocol>,
    //#[clap(default_missing_value = "all")]
    direction: Option<DataDirection>,
}

impl ConnectorsOpt {
    pub fn run(&self) {
        match &self.doc {
            DocScheme::Reference => {
                let (direction, protocol) = if self.direction.is_none() && self.protocol.is_none() {
                    info!("Collecting ref docs for all connectors");
                    (DataDirection::default(), DataProtocol::default())
                } else {
                    let direction = if let Some(d) = self.direction.clone() {
                        d
                    } else {
                        DataDirection::default()
                    };

                    let protocol = if let Some(p) = self.protocol.clone() {
                        p
                    } else {
                        DataProtocol::default()
                    };

                    info!(
                        "Collecting ref {:?} docs {:?} connector",
                        &direction, &protocol
                    );

                    (direction, protocol)
                };

                self.download_connector_ref(direction, protocol);
            }
        }
    }

    fn download_connector_ref(&self, direction: DataDirection, protocol: DataProtocol) {
        info!("Downloading refs");
        if protocol == DataProtocol::All {
            for p in DataProtocol::value_variants() {
                if p.clone() == DataProtocol::All {
                    continue;
                }
                info!("{p:?}")
            }
        }
    }
}
