use std::{
    env::{args, current_dir},
    fs::read_dir,
    path::PathBuf,
};

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

    let dir = read_dir(args.path).expect("Couldn't read target dir");
    let files_with_ext: Vec<PathBuf> = dir
        .filter_map(|dir| {
            let entry = dir.ok()?;
            let path = entry.path();

            if target_extensions.contains(&path.extension()?.to_str()?) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    println!(
        "Found the following files with the selected extensions: {:?}",
        files_with_ext
    );
}
