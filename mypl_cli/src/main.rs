extern crate mypl_lex;

extern crate clap;
extern crate anyhow;

use mypl_lex::prelude::*;

use std::path::Path;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for input in args.input.iter() {
        println!("Tokenizing \"{}\"", input);

        let input_path = Path::new(input);
        let content = read_file(input_path)?;
        let mut tokenizer = Tokenizer::new(&content);
        while let Some(token) = tokenizer.next_token() {
            println!("\ttoken: {:#?}", token);
        }

    }

    println!("done");

    Ok(())
}


fn read_file(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path.as_ref())
        .map_err(anyhow::Error::from)
}
