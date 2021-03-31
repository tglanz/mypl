extern crate log;
extern crate log4rs;
extern crate mypl_lex;

use std::fs;
use std::path::Path;
use std::process;

use create::cli;

fn main() {
    log4rs::init_file("resources/log4rs.yaml", Default::default()).unwrap();
    log::info!("welcome to mypl!\n");

    let arguments = cli::parse();
    cli::exec_default_behaviour(&arguments);

    for input in arguments.inputs.iter() {
        log::debug!("working source file: {}", input);

        let code = try_read_file(Path::new(input));
        log::debug!("code:\n{}", code);

        let tokens = mypl_lex::lexer::Lexer::new(&code).tokenize();
        
        if arguments.show_tokens {
            log::info!("tokens of {:?}:\n{:#?}", input, tokens);
        } else {
            log::debug!("tokens:\n{:#?}", tokens);
        }

        if tokens.iter().any(|token| token.kind.is_invalid()) {
            log::error!("failed to tokenize, found an invalid token at: {:?}", input);
            return;
        }
    }
}

fn try_read_file<P: AsRef<Path>>(path: P) -> String {
    match fs::read_to_string(path.as_ref()) {
        Err(error) => {
            log::error!(
                "failed to read file: `{}`. error: {}. aborting",
                path.as_ref().display(),
                error
            );
            process::exit(1);
        }
        Ok(content) => content,
    }
}
