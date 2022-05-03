use std::{env, fs, path::PathBuf};

fn main() {
    let current_dir = env::current_dir().expect("Could not get current directory.");
    recursive_walk(current_dir, 0)
}

fn recursive_walk(current_dir: PathBuf, depth: u32) {
    let mut paths: Vec<_> = fs::read_dir(current_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|d| !d.file_name().to_string_lossy().starts_with(".")) // hide dotfiles
        .collect();

    paths.sort_by_key(|dir| dir.path());

    for entry in paths {
        let mut spacing = String::with_capacity(depth as usize);
        for _ in 1..depth {
            spacing += "  "
        }

        println!("{}{}", spacing, entry.file_name().to_string_lossy());
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                recursive_walk(entry.path(), depth + 2)
            }
        }
    }
}
