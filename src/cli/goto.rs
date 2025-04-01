use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Goto {
    /// The name of the branch, stack, or commit to switch to
    pub name: String,
}

impl Goto {
    pub async fn run(&self) -> Result<()> {
        app::goto::goto(&self.name)
    }
}