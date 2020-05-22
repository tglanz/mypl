use std::env;
use std::process;
use std::vec::Vec;

#[derive(Debug, Default)]
pub struct Arguments {
    pub abort: bool,
    pub inputs: Vec<String>,
    pub help: bool,
    pub error: Option<String>,
}

pub fn usage() -> String {
    "
usage: program [OPTIONS] {output}

OPTIONS
    -h,--help       enable; show this help message
    -v,--verbose    enable; debug message
    -i,--input      list; input files

POSITIONALS
    output          path; indicates where to save outputs

EXAMPLES
    program -i src/main.pl -i src/calc.pl ./out
    "
    .to_owned()
}

pub fn parse() -> Arguments {
    let mut arguments = Arguments::default();

    let mut env_args = env::args();
    while let Some(key) = env_args.next() {
        if eq_any(&key, &["-h", "--help"]) {
            arguments.abort = true;
            arguments.help = true;
        }

        if eq_any(&key, &["-i", "--input"]) {
            if let Some(value) = env_args.next() {
                arguments.inputs.push(value.to_owned());
            } else {
                arguments.abort = true;
                arguments.help = true;
                arguments.error = Some("missing value for input".to_owned());
            }
        }
    }

    arguments
}

pub fn exec_default_behaviour(args: &Arguments) {
    if args.help {
        println!("{:#}", usage());
    }

    if let Some(error) = &args.error {
        println!("Failed to parse arguments: {:#}", error);
    }

    if args.abort {
        let status = match args.error {
            Some(_) => 1,
            None => 0,
        };

        process::exit(status);
    }
}

fn eq_any(key: &String, choices: &[&str]) -> bool {
    choices.iter().any(|choice| key.eq(choice))
}
