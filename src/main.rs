use clap::Parser;
use zyra::cli::Run;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match zyra::cli::Cmd::parse().run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}
