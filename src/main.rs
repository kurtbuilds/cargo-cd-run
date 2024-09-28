use anyhow::Result as AnyResult;
use clap::Parser;
use std::env::{args_os, set_current_dir};
use std::process::Command;
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
    dbg!(&cli);

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
    let target_dir = std::env::var("CARGO_TARGET_DIR");
    let target_dir = target_dir.as_deref().unwrap_or("target");

    let find = format!("fd . {target_dir}/debug {target_dir}/release -t x --maxdepth 1 -X ls -t | head -n1");
    let bin = Command::new("/bin/bash")
        .args(&["-c", &find])
        .output()?;
    let bin = bin.stdout().trim_end();

    set_current_dir(cli.dir)?;
    Command::new(bin)
        .args(target_args)
        .run()?;
    Ok(())
}
