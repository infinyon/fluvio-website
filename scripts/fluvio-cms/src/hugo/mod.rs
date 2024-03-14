use anyhow::Result;
use clap::Parser;
use xshell::{cmd, Shell};

/// Wrapper around the `hugo` cli in your PATH. Starts development server
#[derive(Clone, Parser, Debug)]
pub struct HugoOpt {}

impl HugoOpt {
    pub fn run(&self) -> Result<()> {
        let sh = Shell::new()?;

        cmd!(
            sh,
            "hugo server --watch --verbose --buildDrafts --cleanDestinationDir --disableFastRender --buildFuture --ignoreCache --baseURL http://localhost --appendPort --navigateToChanged"
        )
        .run()?;
        Ok(())
    }
}
