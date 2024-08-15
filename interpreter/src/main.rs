use std::env;

use parser::to_ast;

use crate::tokens::tokenizer::{from_binary, from_text};

mod engine;
mod instructions;
mod modules;
mod parser;
mod representation;
mod tokens;
mod types;
mod validation;
mod values;

fn main() {
    let is_binary = false;
    let Config { path } = parse_args();
    let tokenized = match is_binary {
        true => {
            let input = std::fs::read(path).expect("failed to read from provided input file");
            from_binary(input)
        }
        false => {
            let input =
                std::fs::read_to_string(path).expect("failed to read from provided input file");
            from_text(input)
        }
    };
    let ast = to_ast(tokenized);
}

const PATH_FLAG: &str = "-p";

fn parse_args() -> Config {
    let mut path = None;
    let args: Vec<String> = env::args().collect();
    args.iter().enumerate().for_each(|(i, arg)| {
        if arg == PATH_FLAG {
            if let Some(p) = args.get(i + 1) {
                path = Some(p.clone());
            }
        }
    });
    Config {
        path: path.expect("no file path supplied"),
    }
}

struct Config {
    path: String,
}
