use dotenv::dotenv;

pub fn load(command: String) -> Result<String, String> {
    dotenv().unwrap();

    let path = match std::env::var("FABRIC_PATH") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("FABRIC_PATH environment variable not set");
            std::process::exit(exitcode::DATAERR);
        }
    };

    let full_path = format!("{}/{}/system.md", path, command);
    let full_path2 = full_path.clone();

    match std::fs::read_to_string(full_path) {
        Ok(content) => {
            // We expect every file to end with something like this:
            //
            //     # INPUT:
            //
            //     Input:
            //
            // Possibly with blank lines and/or whitespace. We want to strip everything
            // after the '# INPUT:' line and the blank line that follows it.

            let mut prompt = String::new();
            for line in content.lines() {
                if line.starts_with("# INPUT:") {
                    break;
                }
                prompt.push_str(line);
                prompt.push_str("\n");
            }
            prompt.push_str("# INPUT:\n\n");
            Ok(prompt)
        }
        Err(_) => {
            let msg = format!("Could not load prompt from '{}'", full_path2);
            Err(msg.to_string())
        }
    }
}