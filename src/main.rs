#![deny(unsafe_code)]
use std::process;

use clap::Parser;
use miette::Result;

use tyupy::{cli::Opts, config::Config, exit_codes::ExitCode, out::stderr};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            stderr(&format!("Error: {:?}", err));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let opts = Opts::parse();
    let _config = construct_config(opts);
    Ok(ExitCode::Success)
}

fn construct_config(opts: Opts) -> Config {
    Config {
        format: opts.format,
    }
}
