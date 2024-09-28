use std::env::{set_current_dir, vars};
use std::process::Command;
use clap::{Parser, Subcommand};
use anyhow::Result as AnyResult;
use std_ext::CommandExt;

#[derive(Parser, Debug)]
struct Args {
    dir: String,
    args1: Vec<String>,
    #[clap(last = true)]
    args2: Vec<String>,
}


fn main() -> AnyResult<()> {
    let cli = Args::parse();

    let cargo_args;
    let target_args;
    if cli.args2.is_empty() {
        cargo_args = Vec::new();
        target_args = cli.args1;
    } else {
        cargo_args = cli.args1;
        target_args = cli.args2;
    }

    Command::new("cargo")
        .arg("build")
        .args(cargo_args)
        .run()?;
    set_current_dir(cli.dir)?;
    let bin_dir = std::env::var("CARGO_TARGET_DIR").as_deref().unwrap_or("target");
    dbg!(vars());
    // Command::new("")
    Ok(())
}
