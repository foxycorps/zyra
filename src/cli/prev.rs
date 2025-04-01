use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Prev {
    /// The number of steps to go back
    #[clap(short, long, default_value = "1")]
    pub steps: usize,
}

impl Prev {
    pub async fn run(&self) -> Result<()> {
        app::prev::prev(self.steps)
    }
}