use anyhow::Result;
use clap::Parser;

use crate::app;

#[derive(Parser, Debug)]
pub struct Restack {
    /// Restack the entire stack
    #[clap(short, long, default_value_t = false)]
    pub all: bool,

    /// Submit the stack after restacking
    #[clap(short, long, default_value_t = false)]
    pub submit: bool,
}

impl Restack {
    pub async fn run(&self) -> Result<()> {
        app::restack::restack(self.all)?;
        if self.submit {
            app::submit::submit(false, false, false).await?;
        }
        Ok(())
    }
}
