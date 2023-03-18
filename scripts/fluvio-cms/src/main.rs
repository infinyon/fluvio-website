pub mod connectors;
use connectors::ConnectorsOpt;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

// Not needed yet
//
///// Refers to the quadrants of Divio documentation system
///// https://documentation.divio.com/
//#[derive(Clone, Debug, ValueEnum, PartialEq)]
//pub enum DocScheme {
//    #[clap(alias = "t")]
//    Tutorials,
//    #[clap(alias = "h")]
//    HowTo,
//    #[clap(alias = "e")]
//    Explanation,
//    #[clap(aliases= ["r", "ref","refs"])]
//    Reference,
//}

/// A specific section or general task to perform on the site
#[derive(Clone, Parser, Debug)]
enum Subject {
    #[clap(visible_aliases = ["connector", "c"])]
    Connectors(ConnectorsOpt),
    //Cli,
    //Api,
    //Docs,
    //#[clap(alias = "sm")]
    //SmartModules,
    //#[clap(alias = "twif")]
    //ThisWeekInFluvio,
    //CheckLinks,
    //Run,
}

impl Subject {
    fn run(&self) -> Result<()> {
        match self {
            Subject::Connectors(opt) => opt.run(),
        }
    }
}

/// Content management CLI for fluvio.io for Fluvio devs
///
/// Use CLI to
/// * Do routine content updates and/or repetitive edits
/// * Run checks in CI for content that requires testing or validation over releases
#[derive(Clone, Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subject: Subject,
}

fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .try_init();

    let args = Args::parse();

    args.subject.run()
}
