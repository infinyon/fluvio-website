use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use xshell::{cmd, Shell};

#[derive(Clone, Parser, Debug)]
pub struct CliOpt {
    #[arg(short, long, default_value = "data/cli-commands.yml")]
    file: PathBuf,
    #[arg(long, default_value = "embeds/cli/help/")]
    out: PathBuf,
    #[arg(long)]
    dry_mode: bool,
}

#[derive(Debug, Deserialize)]
struct CommandsList {
    #[serde(rename = "cli-commands")]
    cli_commands: Vec<String>,
}

struct CommandParsed {
    command: String,
    args: Vec<String>,
}

impl CommandParsed {
    pub fn to_pathbuf(&self) -> PathBuf {
        let mut parts = vec![self.command.clone()];
        parts.extend(self.args.clone());
        let filename = parts.join("-");
        let filename_md = format!("{filename}.md");
        PathBuf::from(filename_md)
    }
}

struct CommandParsedList {
    cli_commands: Vec<CommandParsed>,
}

impl From<CommandsList> for CommandParsedList {
    fn from(value: CommandsList) -> Self {
        let mut parsed_list = Vec::new();
        for c in value.cli_commands.into_iter() {
            let split: Vec<String> = c.split_whitespace().map(|x| x.to_string()).collect();

            parsed_list.push(CommandParsed {
                command: split[0].clone(),
                args: split[1..].to_vec(),
            });
        }

        CommandParsedList {
            cli_commands: parsed_list,
        }
    }
}

impl CliOpt {
    pub fn run(&self) -> Result<()> {
        // open the yaml file
        let sh = Shell::new()?;
        let commands_str = sh.read_file(&self.file)?;
        let commands: CommandParsedList =
            serde_yaml::from_str::<CommandsList>(commands_str.as_str())?.into();

        // Iterate over the cli commands and get the cli help output
        for c in commands.cli_commands.iter() {
            let command = c.command.as_str();
            //let args : Vec<&str> = c.args.clone().iter().map(String::as_str).collect();
            let args: Vec<String> = c.args.clone();

            // run the command
            let output = cmd!(sh, "{command} {args...} --help").read()?;
            let output_md = format!("```\n{output}\n```");

            // Warn: This needs to be consistent between local and ci before it can be relied on
            // write file
            let filepath = &self.out.clone().join(&c.to_pathbuf());
            if !&self.dry_mode {
                let mut file = File::create(&filepath)?;
                file.write_all(output_md.as_bytes())?;
            } else {
                println!("Dry Mode: Write to {}", filepath.display());
            }
        }

        Ok(())
    }
}
