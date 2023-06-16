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
    disable_auto_semicolon: bool,

    #[arg(long, default_value_t = false)]
    interpret: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut interperter = Interperter::new();

    if let Some(input) = &args.input {
        let content = read_file(Path::new(&input))?;
        execute(&mut interperter, &content, &args);
    } else {
        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut content = String::new();
            std::io::stdin().read_line(&mut content)?;

            if content.trim().is_empty() {
                break;
            }

            execute(&mut interperter, &content, &args);
        }
    }

    Ok(())
}

fn execute(interperter: &mut Interperter, content: &str, args: &Args) {
    let mut normalized_content = content.trim().to_string();

    if !args.disable_auto_semicolon && !normalized_content.ends_with(";") {
        normalized_content = format!("{};", content);
    }

    let mut tokenizer = Tokenizer::new(&normalized_content);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        if args.show_tokens {
            println!("\ttoken: {:#?}", token);
        }
        tokens.push(token);
    }

    let mut parser = RecursiveDescentParser::new(&tokens);

    match parser.parse() {
        Err(parse_error) => println!("ParseErrror - {}", parse_error),
        Ok(statements) => {
            if args.show_ast {
                println!("{}", AstFormatter::format_ast(&statements));
            }

            if args.interpret {
                for stmt in statements {
                    match interperter.interpret_stmt(&stmt) {
                        Err(err) => println!("InterperterError - {}", err),
                        _ => {},
                    }
                }
            }
        }
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path.as_ref()).map_err(anyhow::Error::from)
}
