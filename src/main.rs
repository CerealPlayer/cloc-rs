use std::{env::current_dir, fs::read_to_string, path::PathBuf};

use clap::Parser;

use crossbeam_channel::unbounded;
use ignore::{WalkBuilder, WalkState};
use rayon::iter::{ParallelBridge, ParallelIterator};

/// Count lines of code by extension.
#[derive(Parser)]
#[command(version, about = "Count lines of code by extension")]
struct Cli {
    /// Target directory to scan.
    ///
    /// If omitted, uses the current working directory.
    path: Option<PathBuf>,
}

const GIT_IGNORE: &str = ".gitignore";

struct AppArgs {
    path: PathBuf,
}

#[derive(Default, Clone)]
struct LineCount {
    total: usize,
    empty: usize,
}

fn main() {
    let cli = Cli::parse();

    let mut target_dir = current_dir().expect("Failed to get current directory");

    if let Some(p) = cli.path {
        target_dir.push(p)
    }

    let args = AppArgs {
        // pattern: cli.pattern,
        path: target_dir,
    };

    // let target_extensions: Vec<&str> = args.pattern.split(",").collect();
    println!(
        "Reading lines of code in dir {:?}, excluding {GIT_IGNORE} by default",
        &args.path
    );

    let walker = WalkBuilder::new(args.path);
    let (s, r) = unbounded();

    walker.build_parallel().run(move || {
        let s = s.clone();
        Box::new(move |path| {
            if let Ok(dir) = path {
                if dir.file_type().map_or(false, |ft| ft.is_file()) {
                    // println!("Sent file to processing list");
                    s.send(dir.into_path()).unwrap();
                }
            }
            WalkState::Continue
        })
    });

    let total_count: LineCount = r
        .into_iter()
        .par_bridge()
        .filter_map(|p| read_to_string(p).ok())
        .map(|text| {
            let mut count = LineCount::default();

            for line in text.lines() {
                count.total += 1;
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    count.empty += 1;
                }
            }
            // println!("Processed file");
            count
        })
        .reduce(LineCount::default, |a, b| {
            // println!("Reducing...");
            LineCount {
                total: a.total + b.total,
                empty: a.empty + b.empty,
            }
        });

    println!("Total lines {}", total_count.total);
    println!("Empty lines {}", total_count.empty);
}
