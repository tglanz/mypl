use std::iter::*;

mod prelude;
mod cli;
mod tokens;

use prelude::*;

fn main() -> Result<()> {
    let arguments = cli::parse();
    cli::exec_default_behaviour(&arguments);

    let mut tokens = arguments.inputs.iter().map(tokens::tokenize);

    Ok(())
}
