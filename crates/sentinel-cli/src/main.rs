#![forbid(unsafe_code)]

use clap::Parser;
use sentinel_cli::args::{Cli, Commands};
use sentinel_cli::commands::{run_bundle, run_check, run_cost_matrix, run_scan, EXIT_INVALID_ARGS};
use std::env;
use std::process::Command;

#[tokio::main]
async fn main() {
    // Handle --version flag manually to embed git SHA and target triple
    if env::args().any(|arg| arg == "--version") {
        let git_sha = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|o| {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim_end().to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        let target_triple = Command::new("rustc")
            .args(["--print", "host-tuple"])
            .output()
            .ok()
            .and_then(|o| {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim_end().to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        println!(
            "sentinel {} (sha: {}; target: {})",
            env!("CARGO_PKG_VERSION"),
            git_sha,
            target_triple
        );
        std::process::exit(0);
    }

    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(err) => {
            if err.use_stderr() {
                let _ = err.print();
                std::process::exit(EXIT_INVALID_ARGS);
            } else {
                err.exit();
            }
        }
    };

    let exit_code = match cli.command {
        Commands::Check(args) => run_check(args).await,
        Commands::CostMatrix(args) => run_cost_matrix(args).await,
        Commands::Scan(args) => run_scan(args).await,
        Commands::Bundle(args) => run_bundle(args).await,
    };

    std::process::exit(exit_code);
}
