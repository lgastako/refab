use crate::fabric;

pub fn run(command: String) {
    println!("Running command: {}", command);
    // We need to first load the prompt from the fabric directory
    let _os: Option<String> = fabric::load(command);
//    let _s: String = fabric::load(command).to_string();
    // If the prompt does not exist we need to report an error appropriately
    // Assuming the prompt has been successfully loaded, then...
    // We need to read the entire contents of stdin and store it in a variable
    // We need to set some maximum size here, which should be derived from the
    // token limits of the model in use...
    // We need to then pass the prompt and the input to the model
    // We need to then print the output to stdout
    // And exit with a status code of 0
}