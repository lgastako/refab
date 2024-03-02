use std::{future::Future, io::Read, pin::Pin};

use dotenv::dotenv;
use openai::{completions::Completion, set_key};

use crate::fabric;

pub async fn run(command: String) {
    let prompt_eth = fabric::load(command);
    match prompt_eth {
        Ok(prompt) => {
            // TODO We need to set some maximum size here, which should be derived from the
            // token limits of the model in use...
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let output_future: Pin<Box<dyn Future<Output = String>>> = Box::pin(complete(prompt, input));
            let output: String = output_future.await;
            println!("{}", output);
            std::process::exit(exitcode::OK);
        }
        Err(msg) => {
            eprintln!("Could not load prompt: {msg}");
            std::process::exit(exitcode::DATAERR);
        }
    }
}

pub async fn complete(prompt: String, input: String) -> String {
    // TODO move to an init method or something
    dotenv().unwrap();
    let var = "OPENAI_API_KEY";
    match dotenv::var(var) {
        Ok(key) => set_key(key),
        Err(_) => {
            eprintln!("Could not load OpenAI key from environment variable {}", var);
            std::process::exit(exitcode::DATAERR);
        }
    }

    let prompt = format!("{}{}", prompt, input);

    let completion = Completion::builder("gpt-3.5-turbo-instruct")
        .prompt(&prompt)
        .max_tokens(1024)
        .create()
        .await
        .unwrap();

    let output = format!("{}", completion.choices[0].text);

    output
}