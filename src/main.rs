use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    // find the gitignore file and add its contents to the blacklist
    let gitignore = Path::new(".gitignore");
    if gitignore.exists() {
        match read_file(gitignore) {
            Ok(contents) => {
                println!("Found .gitignore file, adding its contents to the blacklist");
                for line in contents.lines() {
                    if !line.starts_with("#") && !line.is_empty() {
                        args.push(line.to_string());
                    }
                }
            }
            Err(_) => (),
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut clipboard_content = String::new();

    'outer: for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip if the path matches any blacklist item
        for arg in &args {
            if path.ends_with(arg) || path.to_str().map_or(false, |s| s.contains(arg)) {
                continue 'outer;
            }
        }

        if path.is_file() {
            match read_file(path) {
                Ok(contents) => {
                    clipboard_content.push_str(&format!("{:?}\n```\n{}\n```\n\n", path, contents));
                }
                Err(_) => continue, // Skip non-text files or errors
            }
        }
    }

    ctx.set_contents(clipboard_content).unwrap();
}

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    match String::from_utf8(contents) {
        Ok(text) => Ok(text),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Not a UTF-8 file",
        )),
    }
}
