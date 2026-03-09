use std::{collections::HashMap, env::current_dir, fs::read_to_string, path::PathBuf};

use clap::Parser;

use crossbeam_channel::unbounded;
use ignore::{WalkBuilder, WalkState};
use rayon::iter::{ParallelBridge, ParallelIterator};

use prettytable::{Table, row};

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

#[derive(Default)]
struct FileCount {
    extension: String,
    lines: usize,
    empty: usize,
    // comments: usize,
}

#[derive(Default, Clone)]
struct Summary {
    total: usize,
    empty: usize,
}

fn main() {
    let cli = Cli::parse();

    let mut target_dir = current_dir().expect("Failed to get current directory");

    if let Some(p) = cli.path {
        target_dir.push(p)
    }

    let args = AppArgs { path: target_dir };

    println!(
        "Reading lines of code in dir {:?}, excluding {GIT_IGNORE} patterns by default",
        &args.path
    );

    let walker = WalkBuilder::new(args.path);
    let (s, r) = unbounded();

    walker.build_parallel().run(move || {
        let s = s.clone();
        Box::new(move |path| {
            if let Ok(dir) = path {
                if dir.file_type().map_or(false, |ft| ft.is_file()) {
                    s.send(dir.into_path()).unwrap();
                }
            }
            WalkState::Continue
        })
    });

    let files_counts = r
        .into_iter()
        .par_bridge()
        .filter_map(|p| {
            let text = read_to_string(&p).ok();
            if let Some(t) = text {
                Some((t, p))
            } else {
                None
            }
        })
        .map(|(text, p)| {
            let mut count = FileCount::default();
            let extension = p.extension().and_then(|e| e.to_str()).unwrap_or_default();
            count.extension = String::from(extension);

            for line in text.lines() {
                count.lines += 1;
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    count.empty += 1;
                }
            }
            count
        });

    let per_ext: HashMap<String, Summary> = files_counts
        .fold(
            || HashMap::new(),
            |mut local, fc| {
                let entry: &mut Summary = local.entry(fc.extension).or_default();
                entry.total += fc.lines;
                entry.empty += fc.empty;
                local
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (ext, sum_b) in b {
                    let entry = a.entry(ext).or_default();
                    entry.total += sum_b.total;
                    entry.empty += sum_b.empty;
                }
                a
            },
        );

    let total_summary: Summary = per_ext.values().fold(Summary::default(), |mut acc, s| {
        acc.total += s.total;
        acc.empty += s.empty;
        acc
    });

    let mut table = Table::new();

    // Header
    table.add_row(row!["Extension", "Lines", "Empty"]);

    // Per extension rows
    for (ext, s) in per_ext.iter() {
        table.add_row(row![ext, r->s.total, s.empty]);
    }

    // Global total row
    table.add_row(row!["TOTAL", r->total_summary.total, total_summary.empty]);

    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.printstd();
}
