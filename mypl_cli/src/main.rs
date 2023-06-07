extern crate mypl_lex;
extern crate mypl_ast;
extern crate mypl_parse;

extern crate anyhow;
extern crate clap;

use mypl_lex::prelude::*;
use mypl_parse::prelude::{Parser, RecursiveDescentParser};

use anyhow::Result;
use clap::Parser as ClapParser;
use std::path::Path;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Vec<String>,

    #[arg(short = 'T', long, default_value_t = true)]
    show_tokens: bool,

    #[arg(short = 'A', long, default_value_t = true)]
    show_ast: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for input in args.input.iter() {
        println!("Tokenizing \"{}\"", input);

        let input_path = Path::new(input);
        let content = read_file(input_path)?;
        let mut tokenizer = Tokenizer::new(&content);

        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            println!("\ttoken: {:#?}", token);
            tokens.push(token);
        }

        let mut parser = RecursiveDescentParser::new(&tokens);
        let ast = parser.parse()?;
        if args.show_ast {
            println!("{:#?}", ast);
        }
    }

    Ok(())
}

fn read_file(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path.as_ref()).map_err(anyhow::Error::from)
}
