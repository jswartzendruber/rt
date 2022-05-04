use std::{env, fs, path::PathBuf, process::exit};

struct Program {
    show_hidden: bool,
    path: Option<PathBuf>,
}

#[derive(Copy, Clone)]
struct FileStats {
    dirs: u32,
    files: u32,
}

fn main() {
    let program = parse_args(env::args().collect());
    let path = match program.path {
        Some(p) => p,
        None => exit(0),
    };

    let stats = recursive_walk(
        path,
        "",
        0,
        program.show_hidden,
        FileStats { dirs: 0, files: 0 },
    );

    println!("{} directories, {} files", stats.dirs, stats.files);
}

fn display_help() {
    println!("USAGE:");
    println!("    rt [OPTIONS] [PATH]");
    println!("");
    println!("OPTIONS:");
    println!("    -h    Displays this menu");
    println!("    -a    Displays files beginning with .");
}

fn parse_args(args: Vec<String>) -> Program {
    let mut show_hidden = false;
    for arg in &args {
        if arg == "-h" || arg == "--help" {
            display_help();
            return Program {
                path: None,
                show_hidden: false,
            };
        } else if arg == "-a" || arg == "--all" {
            show_hidden = true;
        }
    }

    let path = match args.last() {
        Some(p) => PathBuf::from(p.to_owned()),
        None => env::current_dir().expect("Could not get current directory"),
    };

    if args.len() > 2 || (args.len() > 1 && !show_hidden) {
        Program {
            path: Some(PathBuf::from(path)),
            show_hidden,
        }
    } else {
        Program {
            path: Some(env::current_dir().expect("Could not get current directory")),
            show_hidden,
        }
    }
}

fn recursive_walk(
    dir: PathBuf,
    prefix: &str,
    mut depth: u32,
    show_hidden: bool,
    mut stats: FileStats,
) -> FileStats {
    if depth == 0 {
        depth += 1;
        println!(".");
    }

    let mut paths: Vec<_>;

    if show_hidden {
        paths = fs::read_dir(dir)
            .expect("Invalid ReadDir")
            .map(|r| r.expect("Invalid DirEntry"))
            .collect();
    } else {
        paths = fs::read_dir(dir)
            .expect("Invalid ReadDir")
            .map(|r| r.expect("Invalid DirEntry"))
            .filter(|d| !d.file_name().to_string_lossy().starts_with(".")) // hide dotfiles
            .collect();
    }

    paths.sort_by_key(|dir| dir.path());

    for (idx, entry) in paths.iter().enumerate() {
        let is_dir = entry.file_type().expect("Invalid FileType").is_dir();

        if is_dir {
            stats.dirs += 1
        } else {
            stats.files += 1
        }

        if idx == paths.len() - 1 {
            println!("{}└── {}", prefix, entry.file_name().to_string_lossy());
            if is_dir {
                stats = recursive_walk(
                    entry.path(),
                    &format!("{}    ", prefix),
                    depth + 1,
                    show_hidden,
                    stats,
                );
            }
        } else {
            println!("{}├── {}", prefix, entry.file_name().to_string_lossy());
            if is_dir {
                stats = recursive_walk(
                    entry.path(),
                    &format!("{}│   ", prefix),
                    depth + 1,
                    show_hidden,
                    stats,
                );
            }
        }
    }

    stats
}
