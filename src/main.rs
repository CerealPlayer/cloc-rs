use std::{
    env::{args, current_dir},
    fs::read_to_string,
    path::PathBuf,
};

use cloc_rs::{collect_files_with_extensions, is_import_line};

struct Cli {
    pattern: String,
    path: PathBuf,
    excluded_dirs: Vec<String>,
}

#[derive(Default)]
struct LineCounts {
    total: usize,
    empty: usize,
    imports: usize,
}

fn main() {
    let cli_args: Vec<String> = args().skip(1).collect();
    let pattern_arg = cli_args
        .first()
        .cloned()
        .expect("File extension pattern missing");

    let mut index = 1;
    let path_arg = if cli_args.get(index).is_some_and(|arg| !arg.starts_with("--")) {
        let value = cli_args[index].clone();
        index += 1;
        Some(value)
    } else {
        None
    };

    let mut excluded_dirs: Vec<String> = Vec::new();
    while index < cli_args.len() {
        match cli_args[index].as_str() {
            "--exclude" => {
                index += 1;
                let exclude_list = cli_args
                    .get(index)
                    .expect("Missing value for --exclude. Example: --exclude node_modules,test");

                excluded_dirs = exclude_list
                    .split(',')
                    .map(str::trim)
                    .filter(|name| !name.is_empty())
                    .map(str::to_string)
                    .collect();
            }
            unknown => panic!("Unknown argument: {unknown}"),
        }

        index += 1;
    }

    let mut target_dir = current_dir().expect("Failed to get current directory");

    if let Some(p) = path_arg {
        target_dir.push(p)
    }

    let args = Cli {
        pattern: pattern_arg,
        path: target_dir,
        excluded_dirs,
    };

    println!(
        "Reading lines of code for extensions {:?} in dir {:?}",
        &args.pattern, &args.path
    );

    let target_extensions: Vec<&str> = args.pattern.split(",").collect();
    let excluded_dir_refs: Vec<&str> = args.excluded_dirs.iter().map(String::as_str).collect();
    let files_with_ext =
        collect_files_with_extensions(&args.path, &target_extensions, &excluded_dir_refs);

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
