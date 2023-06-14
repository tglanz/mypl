extern crate mypl_lex;
extern crate mypl_ast;
extern crate mypl_parse;
extern crate mypl_interperter;

extern crate anyhow;
extern crate clap;

use std::io::Write;

use mypl_lex::prelude::*;
use mypl_ast::prelude::*;
use mypl_parse::prelude::*;
use mypl_interperter::prelude::*;

use anyhow::Result;
use clap::Parser as ClapParser;
use std::path::Path;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // input: Vec<String>,
    
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short = 'T', long, default_value_t = false)]
    show_tokens: bool,

    #[arg(short = 'A', long, default_value_t = false)]
    show_ast: bool,

    #[arg(long, default_value_t = false)]
    interpret: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(input) = args.input {
        let content = read_file(Path::new(&input))?;
        execute(&content, args.show_tokens, args.show_ast, args.interpret)?;
    } else {
        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut content = String::new();
            std::io::stdin().read_line(&mut content)?;

            if content.trim().is_empty() {
                break;
            }

            execute(&content, args.show_tokens, args.show_ast, args.interpret)?;
        }
    }

    Ok(())
}

fn execute(content: &str, show_tokens: bool, show_ast: bool, interpret: bool) -> Result<()> {
    let mut tokenizer = Tokenizer::new(content);
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token() {
        if show_tokens {
            println!("\ttoken: {:#?}", token);
        }
        tokens.push(token);
    }

    let mut parser = RecursiveDescentParser::new(&tokens);
    let statements = parser.parse()?;
    if show_ast {
        println!("{}", AstFormatter::format_ast(&statements));
    }

    if interpret {
        let mut interperter = Interperter::new();
        for stmt in statements {
            interperter.interpret_stmt(&stmt)?;
        }
    }

    Ok(())
}

fn read_file(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path.as_ref()).map_err(anyhow::Error::from)
}
