use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Restack {
    /// Restack the entire stack
    #[clap(short, long, default_value_t = false)]
    pub all: bool,
}

impl Restack {
    pub async fn run(&self) -> Result<()> {
        app::restack::restack(self.all)
    }
}