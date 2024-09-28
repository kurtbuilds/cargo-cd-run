use anyhow::Result as AnyResult;
use clap::Parser;
use std::env::{args_os, set_current_dir};
use std::io;
use std::io::{BufRead, BufReader};
use std::process::{Command};
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

    let (reader, writer) = os_pipe::pipe()?;

    let mut child = Command::new("cargo")
        .arg("build")
        .arg("--message-format=json-diagnostic-rendered-ansi")
        .arg("--color=always")
        .args(cargo_args)
        .stderr(writer.try_clone()?)
        .stdout(writer)
        .spawn()?;

    let buf = BufReader::new(reader);
    let mut bin = None;
    for line in buf.lines() {
        let line = line?;
        if line.starts_with('{') {
            let data: serde_json::Value = serde_json::from_str(&line)?;
            if let Some(exec) = data["executable"].as_str() {
                bin = Some(exec.to_string());
            }
            let Some(message) = data["message"]["rendered"].as_str() else {
                continue;
            };
            print!("{}", message);
        } else {
            println!("{}", line);
        }
    }
    child.wait()?;

    let bin = bin.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No executable found"))?;
    set_current_dir(cli.dir)?;
    Command::new(bin)
        .args(target_args)
        .run()?;
    Ok(())
}
