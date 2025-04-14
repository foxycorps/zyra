pub use crate::cli::cmd::*;

use anyhow::Result;

pub mod branch;
mod cmd;
pub mod goto;
pub mod init;
pub mod log;
pub mod next;
pub mod prev;
pub mod restack;
pub mod submit;

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
            Cmd::Restack(restack) => restack.run().await,
            Cmd::Submit(submit) => submit.run().await,
        }
    }
}
