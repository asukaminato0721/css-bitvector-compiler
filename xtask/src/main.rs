mod benchmark;
mod check;
mod context;
mod corpus;
mod report;
mod run;
mod stats;

use context::Workspace;
use std::{env, error::Error};

fn main() {
    if let Err(error) = dispatch() {
        eprintln!("xtask failed: {error}");
        std::process::exit(1);
    }
}

fn dispatch() -> Result<(), Box<dyn Error>> {
    let workspace = Workspace::discover()?;
    let mut arguments = env::args().skip(1);
    let Some(command) = arguments.next() else {
        print_help();
        return Ok(());
    };
    let rest: Vec<_> = arguments.collect();
    match command.as_str() {
        "check" => check::execute(&workspace),
        "corpus" => corpus::execute(&workspace),
        "run" => run::execute(&workspace, &rest),
        "benchmark" | "bench" => benchmark::execute(&rest),
        "stats" => stats::execute(&workspace, &rest),
        "report" => report::execute(&workspace, &rest),
        "all" => {
            check::execute(&workspace)?;
            corpus::execute(&workspace)?;
            benchmark::execute(&["--site".into(), "testcase".into()])
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        other => Err(format!("unknown xtask command `{other}`; run `cargo xtask help`").into()),
    }
}

fn print_help() {
    println!(
        "\
CSS bitvector development tasks

Usage:
  cargo xtask check
  cargo xtask corpus
  cargo xtask run --site <name> [--update]
  cargo xtask run --all [--update]
  cargo xtask benchmark --site <name> [engine,...]
  cargo xtask stats --site <name>
  cargo xtask stats --css <path>
  cargo xtask report [--base <directory>] [--output-dir <directory>]
  cargo xtask all

`run` compares naive, bit, tri, and rec_tri in memory. It writes consolidated
results only when --update is present."
    );
}
