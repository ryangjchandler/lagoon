use std::fs::read_to_string;
use clap::{Arg, App, AppSettings};

use lagoon_parser::{generate, parse};
use lagoon_interpreter::{interpret};
use lagoon_vm::{execute};

mod cmd;

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
                .arg(
                    Arg::new("vm")
                        .about("Execute the script using the virtual machine (experimental).")
                        .long("vm")
                        .takes_value(false)
                )
        )
        .subcommand(
            App::new("js")
                .about("Transpile a Lagoon script to JavaScript.")
                .version(VERSION)
                .arg(
                    Arg::new("file")
                        .about("The Lagoon file to transpile.")
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .about("The target destination for the transpiled file.")
                        .required(true)
                )
        )
        .get_matches();

    if let Some(ref run) = matches.subcommand_matches("run") {
        let file = run.value_of("file").unwrap();
        let path = std::path::PathBuf::from(file);
        let contents = read_to_string(file).unwrap();
        let tokens = generate(contents.as_str());
        
        match parse(tokens) {
            Ok(ast) => {
                match run.is_present("vm") {
                    true => {
                        match execute(ast) {
                            _ => (),
                        };
                    },
                    false => {
                        match interpret(ast, path) {
                            Ok(_) => {},
                            Err(e) => e.print(),
                        };
                    }
                }
            },
            Err(e) => e.print(),
        };
    } else if let Some(ref js) = matches.subcommand_matches("js") {
        let file = js.value_of("file").unwrap();
        let contents = read_to_string(file).unwrap();
        let output = js.value_of("output").unwrap();
        let tokens = generate(contents.as_str());

        match parse(tokens) {
            Ok(ast) => {
                match cmd::js(ast, output) {
                    Ok(_) => {},
                    Err(e) => e.print(), 
                };
            },
            Err(e) => e.print(),
        };
    }
}
