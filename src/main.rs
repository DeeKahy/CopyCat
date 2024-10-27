use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, fs, path::Path, process::Command};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut blacklist = Vec::new();
    let mut default_excludes = Vec::new();

    // Process arguments
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-d" {
            // Add default excludes if specified
            default_excludes.push("README.md".to_string());
            default_excludes.push(".lock".to_string());
            default_excludes.push("target/".to_string());
            // You can add more default excludes here
            i += 1;
        } else {
            blacklist.push(args[i].clone());
            i += 1;
        }
    }

    // Combine default excludes with blacklist
    blacklist.extend(default_excludes);

    // Add gitignore patterns to blacklist
    if let Ok(contents) = fs::read_to_string(".gitignore") {
        blacklist.extend(
            contents.lines()
                .filter(|line| !line.starts_with('#') && !line.is_empty())
                .map(String::from)
        );
    }

    // Collect file contents
    let mut clipboard_content = String::new();
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || blacklist.iter().any(|arg| 
            path.ends_with(arg) || path.to_str().map_or(false, |s| s.contains(arg))) {
            continue;
        }

        if let Ok(contents) = fs::read_to_string(path) {
            clipboard_content.push_str(&format!("{:?}\n```\n{}\n```\n\n", path, contents));
        }
    }

    // Try setting clipboard content using multiple methods
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    if let Err(e) = ctx.set_contents(clipboard_content.clone()) {
        eprintln!("cli-clipboard failed: {:?}", e);

        // Fallback to wl-copy
        if let Err(e) = Command::new("wl-copy")
            .arg(&clipboard_content)
            .status() {
            eprintln!("wl-copy failed: {}", e);
        }
    }

    // Quick verification
    if let Ok(output) = Command::new("wl-paste").output() {
        if output.status.success() {
            println!("Clipboard content set successfully");
        }
    }

    Ok(())
}
