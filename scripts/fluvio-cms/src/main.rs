pub mod connectors;
use connectors::{ConnectorsOpt, DataProtocol};

use clap::{Parser, ValueEnum};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Refers to the quadrants of Divio documentation system
/// https://documentation.divio.com/
#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum DocScheme {
    //#[clap(alias = "t")]
    //Tutorials,
    //#[clap(alias = "h")]
    //HowTo,
    //#[clap(alias = "e")]
    //Explanation,
    #[clap(aliases= ["r", "ref","refs"])]
    Reference,
}

/// A specific section or general task to perform on the site
#[derive(Clone, Parser, Debug)]
enum Subject {
    #[clap(alias = "c")]
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
    fn run(&self) {
        match self {
            Subject::Connectors(opt) => opt.run(),
        }
    }
}

#[derive(Clone, Parser, Debug)]
//#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subject: Subject,
}

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .try_init();

    let args = Args::parse();

    args.subject.run()

    /*
    Idea is we'll have a list of where to get connector readme (repo first, but long-term from Hub package)

    Then we'll do any link fixing before saving to disk
     */
}
