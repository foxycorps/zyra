use crate::app::log::log;
use anyhow::Result;
use clap::Parser;

use super::Run;

#[derive(Parser, Debug)]
pub struct Log {
    /// Show the graph of the stack
    #[clap(short, long)]
    pub graph: bool,

    #[clap(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Show the stack in json format
    #[clap(short, long, default_value_t = false)]
    pub json: bool,
}

impl Run for Log {
    async fn run(&self) -> Result<()> {
        log(self.graph, self.verbose, self.json)
    }
}
