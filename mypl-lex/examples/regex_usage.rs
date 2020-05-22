extern crate regex;

use std::fs;
use regex::Regex;

fn main() {
    showcase_2();
}

fn showcase_1() {
    let content = "// this is some comment\n// this is another comment";
    let regex = Regex::new(r"//(.*)\n").unwrap();
    if let Some(captures) = regex.captures(content) {
        let whole = captures.get(0).unwrap();
        let message = captures.get(1).unwrap();
        println!(
            "whole: {:#?}, message: {:#?}",
            whole.as_str(),
            message.as_str(),
        );
    }
}

fn showcase_2() {
    let regex = Regex::new(r"^[\t\n\r\f]+").unwrap();
    let content = fs::read_to_string("resources/code-examples/a.mypl").unwrap();

    let captures = regex.captures(&content).map(|c| {
        println!("captures: {:#?}", c);
    });
}