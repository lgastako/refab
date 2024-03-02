pub fn load(command: String) -> Result<String, String> {
    // TODO unhardcode the base path
    let path = "/Users/john/src/fabric/patterns";
    let full_path = format!("{}/{}/system.md", path, command);
    let full_path2 = full_path.clone();
    // println!("Full path: {}", full_path);
    match std::fs::read_to_string(full_path) {
        Ok(content) => {
            //   Ok(content),
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
                println!("LINE: {}", line);
                if line.starts_with("# INPUT:") {
                    prompt.push_str(line);
                    prompt.push_str("\n");
                    println!("breaking");
                    break;
                }
                println!("pushing");
                prompt.push_str(line);
                prompt.push_str("\n");
            }
            Ok(prompt)
        }
        Err(_) => {
            let msg = format!("Could not load prompt from '{}'", full_path2);
            Err(msg.to_string())
        }
    }
}