use std::io::Read;

use openai::{chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole}, set_key};

use crate::fabric;

pub async fn run(command: String) {
    let prompt_eth = fabric::load(command);
    match prompt_eth {
        Ok(prompt) => {
            // TODO We need to set some maximum size here, which should be derived from the
            // token limits of the model in use...
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).expect("failed to read input from stdin");

            let output: String = complete(prompt, input).await;
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
    let key_var   = "REFAB_OPENAI_API_KEY";
    let model_var = "REFAB_OPENAI_MODEL";

    let default_model = "gpt-3.5-turbo-instruct".to_string();

    let model = match dotenv::var(model_var) {
        Ok(model) => model,
        Err(_) => default_model
    };

    match dotenv::var(key_var) {
        Ok(key) => set_key(key),
        Err(_) => {
            eprintln!("Could not load OpenAI key from environment variable {}", key_var);
            std::process::exit(exitcode::DATAERR);
        }
    }

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(prompt),
        name: None,
        function_call: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(input),
        name: None,
        function_call: None,
    });

    let chat_completion = ChatCompletion::builder(&model, messages.clone())
            .create()
            .await
            .expect("ChatCompletion call to OpenAI failed");

    let returned_message = chat_completion.choices
            .first()
            .expect("Not chat completion choices returned")
            .message
            .clone();

    let output = returned_message.content.clone().unwrap().trim().to_string();

    output
}