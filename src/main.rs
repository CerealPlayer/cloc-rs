use std::{
    env::{args, current_dir},
    fs::read_to_string,
    path::PathBuf,
};

use cloc_rs::collect_files_with_extensions;

struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let pattern_arg = args().nth(1).expect("File extension pattern missing");
    let path_arg = args().nth(2);
    let mut target_dir = current_dir().expect("Failed to get current directory");

    if let Some(p) = path_arg {
        target_dir.push(p)
    }

    let args = Cli {
        pattern: pattern_arg,
        path: target_dir,
    };

    println!(
        "Reading lines of code for extensions {:?} in dir {:?}",
        &args.pattern, &args.path
    );

    let target_extensions: Vec<&str> = args.pattern.split(",").collect();
    let files_with_ext = collect_files_with_extensions(&args.path, &target_extensions);

    println!(
        "Found the following files with the selected extensions: {:?}",
        files_with_ext
    );

    let total_lines: usize = files_with_ext
        .iter()
        .filter_map(|path| read_to_string(path).ok())
        .map(|content| content.lines().count())
        .sum();

    files_with_ext.iter().for_each(|f| {
        let content = read_to_string(f);
        if let Ok(text) = content {
            println!("File with {} lines", text.lines().count())
        }
    });

    println!("Total lines of code: {}", total_lines);
}
