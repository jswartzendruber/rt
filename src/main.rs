use std::{env, fs, path::PathBuf};

fn main() {
    let current_dir = env::current_dir().expect("Could not get current directory.");
    recursive_walk(current_dir, "", 0)
}

fn recursive_walk(dir: PathBuf, prefix: &str, mut depth: u32) {
    // display root
    if depth == 0 {
        depth += 1;
        println!(".");
    }

    let mut paths: Vec<_> = fs::read_dir(dir)
        .expect("Invalid ReadDir")
        .map(|r| r.expect("Invalid DirEntry"))
        .filter(|d| !d.file_name().to_string_lossy().starts_with(".")) // hide dotfiles
        .collect();

    paths.sort_by_key(|dir| dir.path());

    for (idx, entry) in paths.iter().enumerate() {
        if idx == paths.len() - 1 {
            println!("{}└── {}", prefix, entry.file_name().to_string_lossy());
            if entry.file_type().expect("Invalid FileType").is_dir() {
                recursive_walk(entry.path(), &format!("{}    ", prefix), depth + 1);
            }
        } else {
            println!("{}├── {}", prefix, entry.file_name().to_string_lossy());
            if entry.file_type().expect("Invalid FileType").is_dir() {
                recursive_walk(entry.path(), &format!("{}│   ", prefix), depth + 1);
            }
        }
    }
}
