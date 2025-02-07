use crate::errors::*;
use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Increase logging output (can be used multiple times)
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
    /// Change the current directory to this path before executing the subcommand
    #[arg(short = 'C', long)]
    pub context: Option<PathBuf>,
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Build(Build),
    Update(Update),
    Completions(Completions),
}

/// Run a build in a reproducible environment
#[derive(Debug, Parser)]
pub struct Build {
    /// The dependency lockfile to use
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Do not delete the build container, wait for ctrl-c
    #[arg(short, long)]
    pub keep: bool,
    /// The command to execute inside the build container
    #[arg(required = true)]
    pub cmd: Vec<String>,
}

/// Update all dependencies of the reproducible environment
#[derive(Debug, Parser)]
pub struct Update {
    /// Do not attempt to pull the container tag from registry before resolving it
    #[arg(long)]
    pub no_pull: bool,
    /// Do not delete the build container, wait for ctrl-c
    #[arg(short, long)]
    pub keep: bool,
}

/// Generate shell completions
#[derive(Debug, Parser)]
pub struct Completions {
    pub shell: Shell,
}

impl Completions {
    pub fn generate<W: io::Write>(&self, mut w: W) -> Result<()> {
        clap_complete::generate(self.shell, &mut Args::command(), "repro-env", &mut w);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zsh_completions() {
        Completions { shell: Shell::Zsh }
            .generate(io::sink())
            .unwrap();
    }
}
