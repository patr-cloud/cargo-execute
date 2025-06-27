# Cargo Execute

A simple tool to execute terminal commands with Cargo, allowing you to run terminal commands as if they were Cargo subcommands.

## Installation

You can install `cargo-execute` using Cargo:

```bash
cargo install cargo-execute
```

## Usage

You can use `cargo-execute` to run any terminal command. For example:

```bash
cargo execute ls -l
```
This will execute the `ls -l` command in the current directory.
You can also use it to run commands with arguments:

```bash
cargo execute echo "Hello, World!"
```
This will execute the `echo "Hello, World!"` command.
You can also use it to run commands with options:

## Why use this?

This is useful for complex build scripts or when you want to run commands that are not directly related to Cargo but still want to allow other developers to run them easily as part of the Cargo workflow.

