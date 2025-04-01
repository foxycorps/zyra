use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Next;

impl Next {
    pub async fn run(&self) -> Result<()> {
        app::next::next()
    }
}