pub fn load(command: String) -> Result<String, String> {
    let var = "REFAB_FABRIC_PATTERNS_PATH".to_string();
    let error_msg = format!("{} environment variable", var);
    let path = std::env::var(var).expect(&error_msg);
    let full_path = format!("{}/{}/system.md", path, command);

    match std::fs::read_to_string(&full_path) {
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
            let msg = format!("Could not load prompt from '{}'", full_path);
            Err(msg.to_string())
        }
    }
}