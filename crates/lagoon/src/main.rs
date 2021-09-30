use std::fs::read_to_string;
use clap::{Arg, App, AppSettings};

use lagoon_parser::{generate, parse};
use lagoon_interpreter::{interpret};

const VERSION: &str = "0.1-beta";

fn main() {
    let matches = App::new("Lagoon")
        .version(VERSION)
        .author("Ryan Chandler <lagoon@ryangjchandler.co.uk>")
        .about("The official interpreter for Lagoon.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("run")
                .about("Run a Lagoon file.")
                .version(VERSION)
                .arg(
                    Arg::new("file")
                        .about("The Lagoon file to execute.")
                        .required(true)
                )
        )
        .get_matches();

    if let Some(ref run) = matches.subcommand_matches("run") {
        let file = run.value_of("file").unwrap();
        let contents = read_to_string(file).unwrap();
        let tokens = generate(contents.as_str());
        
        match parse(tokens) {
            Ok(ast) => {
                match interpret(ast) {
                    Ok(_) => {},
                    Err(e) => e.print(),
                };
            },
            Err(e) => e.print(),
        };
    }
}
