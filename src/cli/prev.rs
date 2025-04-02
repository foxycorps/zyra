use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Prev;

impl Prev {
    pub async fn run(&self) -> Result<()> {
        app::prev::prev()
    }
}