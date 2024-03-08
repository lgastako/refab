use std::{collections::HashSet, error::Error, fs::{DirEntry, File}, io::BufReader, path::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cmd {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<String>,
    pub system_prompt: String,
    pub user_prompt: Option<String>,
}

pub fn load(path: &str) -> Result<Cmd, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cmd: Cmd = serde_json::from_reader(reader)?;
    Ok(cmd)
}

pub fn list() -> () {
    let var = "REFAB_FABRIC_PATTERNS_PATHS".to_string();
    let error_msg = format!("{} environment variable", var);
    let paths = std::env::var(var).expect(&error_msg);
    let mut seen: HashSet<String> = HashSet::new();

    for path in paths.split(";") {
        let check_path = Path::new(path);
        if std::path::Path::exists(&check_path) {
            println!("FROM PATH: {}", path);

            let dir_read: std::fs::ReadDir =
                std::fs::read_dir(path).expect("Could not read directory");
            let mut entries: Vec<DirEntry> = dir_read
                .collect::<Result<Vec<DirEntry>, std::io::Error>>()
                .expect("Could not collect entries");
            entries.sort_by(compare_dir_entries);
            for entry in entries {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().expect("Could not get file name");
                    let name_str = name.to_str().expect("Could not convert name to string");
                    if name_str.starts_with(".") {
                        continue
                    }
                    if seen.contains(name_str) {
                        println!("  {} (shadowed)", name.to_str().expect("Could not convert to string"));
                    } else {
                        println!("  {}", name.to_str().expect("Could not convert to string"));
                    }
                    seen.insert(name_str.to_owned());
                }
            }
        }
    }
}

fn compare_dir_entries(a: &DirEntry, b: &DirEntry) -> std::cmp::Ordering {
    let a = a.path();
    let b = b.path();
    let a = a.file_name().expect("Could not get file name");
    let b = b.file_name().expect("Could not get file name");
    a.cmp(&b)
}