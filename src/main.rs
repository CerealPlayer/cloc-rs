use std::{
    env::current_dir,
    fs::read_to_string,
    path::PathBuf,
};

use clap::Parser;

use cloc_rs::{collect_files_with_extensions, is_import_line};

/// Count lines of code by extension.
#[derive(Parser)]
#[command(version, about = "Count lines of code by extension")]
struct Cli {
    /// Comma-separated file extensions (example: `rs,ts,js`).
    pattern: String,

    /// Target directory to scan.
    ///
    /// If omitted, uses the current working directory.
    path: Option<PathBuf>,

    /// Comma-separated directory names to exclude recursively.
    ///
    /// Example: `--exclude node_modules,test`
    #[arg(long = "exclude", value_delimiter = ',')]
    exclude: Vec<String>,
}

struct AppArgs {
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
    let cli = Cli::parse();

    let mut target_dir = current_dir().expect("Failed to get current directory");

    if let Some(p) = cli.path {
        target_dir.push(p)
    }

    let args = AppArgs {
        pattern: cli.pattern,
        path: target_dir,
        excluded_dirs: cli.exclude,
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
