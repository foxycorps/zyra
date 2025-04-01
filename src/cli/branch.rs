use super::Run;
use crate::app;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Branch {
    pub name: String,

    #[clap(short, long)]
    pub from: Option<String>,

    #[clap(short, long, default_value = "false")]
    pub verbose: bool,
}

impl Run for Branch {
    async fn run(&self) -> Result<()> {
        app::branch::branch(self.name.clone(), self.from.clone(), self.verbose)
    }
}
