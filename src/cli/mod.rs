pub use crate::cli::cmd::*;

use anyhow::Result;

pub mod branch;
mod cmd;
pub mod init;
pub mod log;
pub mod prev;
pub mod next;
pub mod goto;

pub trait Run {
    async fn run(&self) -> Result<()>;
}

impl Run for Cmd {
    async fn run(&self) -> Result<()> {
        match self {
            Cmd::Init(init) => init.run().await,
            Cmd::Branch(branch) => branch.run().await,
            Cmd::Prev(prev) => prev.run().await,
            Cmd::Log(log) => log.run().await,
            Cmd::Next(next) => next.run().await,
            Cmd::Goto(goto) => goto.run().await,
        }
    }
}
