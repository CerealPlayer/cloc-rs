use std::{fs::read_dir, path::{Path, PathBuf}};

pub fn collect_files_with_extensions(root: &Path, target_extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs_to_visit = vec![root.to_path_buf()];

    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
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