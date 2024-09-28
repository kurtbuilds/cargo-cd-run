use anyhow::Result as AnyResult;
use clap::Parser;
use std::env::{args_os, set_current_dir};
use std::process::{Command, Stdio};
use std_ext::{CommandExt, OutputExt};

#[derive(Parser, Debug)]
struct Args {
    dir: String,
    args1: Vec<String>,
    #[clap(last = true)]
    args2: Vec<String>,
}


fn main() -> AnyResult<()> {
    let args = args_os().skip(1);
    let cli = Args::parse_from(args);
    let cargo_args;
    let target_args;
    if cli.args2.is_empty() {
        cargo_args = Vec::new();
        target_args = cli.args1;
    } else {
        cargo_args = cli.args1;
        target_args = cli.args2;
    }
    let output = Command::new("cargo")
        .arg("build")
        .arg("--message-format=json")
        .args(cargo_args)
        .stderr(Stdio::inherit())
        .output()?;
    let output = output.stdout();
    let mut lines = output.lines();
    _ = lines.next_back(); // skip the last line
    let last_line = lines.next_back();
    let data: serde_json::Value = serde_json::from_str(last_line.unwrap())?;
    let bin = data["executable"].as_str().expect("No executable in cargo build output");

    set_current_dir(cli.dir)?;
    Command::new(bin)
        .args(target_args)
        .run()?;
    Ok(())
}
