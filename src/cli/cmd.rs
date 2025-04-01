use clap::Parser;

#[derive(Parser, Debug)]
pub enum Cmd {
    /// Initialize a new stack
    #[clap(alias = "i")]
    Init(crate::cli::init::Init),

    /// Create a new branch
    #[clap(alias = "b")]
    Branch(crate::cli::branch::Branch),

    /// Display the current stack
    #[clap(alias = "l")]
    Log(crate::cli::log::Log),

    /// Navigate to the previous branch in the stack
    #[clap(alias = "p")]
    Prev(crate::cli::prev::Prev),

    /// Navigate to the next branch in the stack
    #[clap(alias = "n")]
    Next(crate::cli::next::Next),

    /// Navigate to a branch, stack, or commit
    #[clap(alias = "g")]
    Goto(crate::cli::goto::Goto),
}
