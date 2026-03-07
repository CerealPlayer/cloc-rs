use std::{fs::read_dir, path::{Path, PathBuf}};

pub fn is_import_line(extension: &str, trimmed_line: &str) -> bool {
    match extension {
        "rs" => trimmed_line.starts_with("use "),
        "js" | "ts" => trimmed_line.starts_with("import ") && trimmed_line.contains(" from "),
        _ => false,
    }
}

pub fn collect_files_with_extensions(
    root: &Path,
    target_extensions: &[&str],
    excluded_dirs: &[&str],
) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs_to_visit = vec![root.to_path_buf()];

    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default();

                    if excluded_dirs.contains(&dir_name) {
                        continue;
                    }

                    dirs_to_visit.push(path);
                    continue;
                }

                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if target_extensions.contains(&ext) {
                        files.push(path);
                    }
                }
            }
        }
    }

    files
}