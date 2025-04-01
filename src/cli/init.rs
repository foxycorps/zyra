use super::Run;
use crate::git::*;
use crate::{app::init, errors};
use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Init {
    pub name: String,

    #[clap(short, long)]
    pub base: Option<String>,

    #[clap(short, long, default_value = "false")]
    pub verbose: bool,
}

impl Run for Init {
    async fn run(&self) -> Result<()> {
        if !repo::is_repo()? {
            return Err(errors::git::GitError::NotGitRepository.into());
        }
        println!("get_branch_name: {:?}", branch::get_branch_name()?);
        init::init(self.name.clone(), self.base.clone(), self.verbose)
    }
}
