use crate::app;

use super::Run;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Submit {
    /// Submit all branches in the stack
    #[clap(short, long)]
    all: bool,

    /// Force push to remote
    #[clap(short, long)]
    force: bool,

    /// Do not push to remote
    #[clap(short, long)]
    no_push: bool,
}

impl Run for Submit {
    async fn run(&self) -> Result<()> {
        app::submit::submit(self.all, self.force, self.no_push).await
    }
}
