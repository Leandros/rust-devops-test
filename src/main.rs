// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use std::error::Error;

use clap::{Arg, ArgAction, Command};
use doxyme_rust_test::subcommands::deploy::deploy;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn init(verbose: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
    SimpleLogger::new()
        .with_level(if verbose {
            LevelFilter::Trace
        } else {
            LevelFilter::Warn
        })
        .with_module_level("tracing::span", LevelFilter::Warn)
        .with_module_level("aws_config", LevelFilter::Warn)
        .with_module_level("aws_smithy_http_tower", LevelFilter::Warn)
        .with_module_level("hyper", LevelFilter::Warn)
        .with_module_level("rustls", LevelFilter::Warn)
        .with_module_level("aws_endpoint", LevelFilter::Warn)
        .init()
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let matches = Command::new("doxyme-rust-test")
        .version("1.0")
        .author("John Doe <john.doe@doxy.me>")
        .about("Only imagination can stop you!")
        .arg(
            Arg::new("verbose")
                .help("Enable verbose logging")
                .short('v')
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("deploy")
                .arg(
                    Arg::new("to")
                        .long("to")
                        .help("Which environment to deploy to")
                        .value_parser(["dev", "staging", "pord"])
                        .required(true)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("project")
                        .help("The project to deploy")
                        .required(true)
                        .value_parser(["frontend", "backend"])
                        .action(ArgAction::Set),
                ),
        )
        .get_matches();

    init(matches.get_flag("verbose"))?;

    match matches.subcommand() {
        Some(("deploy", matches)) => {
            let to = matches.get_one::<String>("to").expect("required");
            let project = matches.get_one::<String>("project").expect("required");
            deploy(to, project).await?;
        }
        _ => unreachable!("did you forget to add the subcommand to the match?"),
    }

    // Doxy.me Senior DevOps Coding Challenge:
    //
    // You're looking at a Rust CLI application to interact with AWS.
    //
    // It's using https://clap.rs to parse and process command line arguments.
    // Furthermore, the application is using Rust's async and depends on
    // the https://tokio.rs/ runtime, and the AWS Rust SDK. For any further
    // dependencies check the `Cargo.toml`.
    //
    //
    // Task:
    //
    // Your task is to add a new sub-command called 'upload'. This command must
    // take a single parameter, the path to a file (e.g., `rust-test upload <path-to-file>`).
    // The file specified must be uploaded to an AWS S3 bucket, using the already
    // provided interface in `src/provider/s3_trait.rs`.
    // The file must be renamed and uploaded to S3 as `<unixtimestamp>.<original-file-ending>`.
    //
    // ** Please implement the subcommand and the required tests. **
    //
    //
    // Further information:
    //
    // The S3 trait has two implementations, the first implementation (`src/provider/s3.rs`)
    // is using the aws-sdk-s3 crate, the second implementation (`src/provider/s3_mock.rs`)
    // is a mock implementation to be used for testing. It's guaranteed to make
    // no network requests.

    Ok(())
}
