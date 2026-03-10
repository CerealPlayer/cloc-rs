use std::{collections::HashMap, env::current_dir, fs::read_to_string, path::PathBuf};

use clap::Parser;

use cloc_rs::{Count, processor::get_processor};
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
            let extension = p
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default()
                .to_string();
            let mut processor = get_processor(&extension);
            let count = processor.count(&text);
            (extension, count)
        });

    let per_ext: HashMap<String, Count> = files_counts
        .fold(
            || HashMap::new(),
            |mut local, (ext, count): (String, Count)| {
                let entry: &mut Count = local.entry(ext).or_default();
                entry.add_count(&count);
                local
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (ext, sum_b) in b {
                    let entry = a.entry(ext).or_default();
                    entry.add_count(&sum_b);
                }
                a
            },
        );

    let total_summary: Count = per_ext.values().fold(Count::default(), |mut acc, s| {
        acc.add_count(s);
        acc
    });

    let mut table = Table::new();

    // Header
    table.add_row(row!["Extension", "Lines", "Empty", "Comments", "Imports"]);

    // Per extension rows
    for (ext, s) in per_ext.iter() {
        table.add_row(row![if ext.is_empty() { "No ext" } else { ext }, r->s.lines, s.empty, s.comments, s.imports]);
    }

    // Global total row
    table.add_row(row!["TOTAL", r->total_summary.lines, total_summary.empty, total_summary.comments, total_summary.imports]);

    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    table.printstd();
}
