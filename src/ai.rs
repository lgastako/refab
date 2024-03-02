use std::io::Read;

use crate::fabric;

pub fn run(command: String) {
    let prompt_eth = fabric::load(command);
    match prompt_eth {
        Ok(prompt) => {
            // TODO We need to set some maximum size here, which should be derived from the
            // token limits of the model in use...
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let output = complete(prompt, input);
            println!("{}", output);
            std::process::exit(exitcode::OK);
        }
        Err(msg) => {
            eprintln!("Could not load prompt: {msg}");
            std::process::exit(exitcode::DATAERR);
        }
    }
}

pub fn complete(prompt: String, input: String) -> String {
    let output = format!("TEMPORARY Prompt: {}\nInput: {}", prompt, input);
    output
}