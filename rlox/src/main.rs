mod runner;
mod scanner;
mod token;
mod token_type;
mod error;

use std::env;
use runner::Runner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut runner = Runner::new();
    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        runner.run_file(&args[1])?;
    } else {
        runner.run_prompt()?;
    }
    Ok(())
}

