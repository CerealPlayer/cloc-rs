use std::{
    env::{args, current_dir},
    fs::read_to_string,
    path::PathBuf,
};

use cloc_rs::{collect_files_with_extensions, is_import_line};

struct Cli {
    pattern: String,
    path: PathBuf,
}

#[derive(Default)]
struct LineCounts {
    total: usize,
    empty: usize,
    imports: usize,
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

    let mut counts = LineCounts::default();

    files_with_ext.iter().for_each(|file_path| {
        if let Ok(text) = read_to_string(file_path) {
            let extension = file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default();

            text.lines().for_each(|line| {
                counts.total += 1;
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    counts.empty += 1;
                } else if is_import_line(extension, trimmed) {
                    counts.imports += 1;
                }
            });
        }
    });

    let lines_without_spaces_and_imports = counts
        .total
        .saturating_sub(counts.empty + counts.imports);

    println!("Total lines of code: {}", counts.total);
    println!("Total empty lines: {}", counts.empty);
    println!("Total import lines: {}", counts.imports);
    println!(
        "Total lines of code minus empty and imports: {}",
        lines_without_spaces_and_imports
    );
}
