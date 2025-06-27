#![doc = include_str!("../README.md")]

use std::{
    os::unix::process::ExitStatusExt,
    process::{self, ExitCode, Stdio},
};

fn main() -> ExitCode {
    let mut args = std::env::args();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args.next().unwrap());

        return ExitCode::FAILURE;
    }

    let Some(command) = args.nth(1) else {
        eprintln!("No command provided.");
        eprintln!("Usage: {} <command> [args...]", args.next().unwrap());
        return ExitCode::FAILURE;
    };

    process::exit(
        std::process::Command::new(command)
            .args(args)
            .envs(std::env::vars())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .status()
            .expect("Failed to execute command")
            .into_raw(),
    );
}
