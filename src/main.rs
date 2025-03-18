use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, fs, path::Path, process::Command, thread, time::Duration};
use walkdir::WalkDir;

fn set_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // First attempt: cli-clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    if let Err(e) = ctx.set_contents(content.to_string()) {
        eprintln!("cli-clipboard failed: {:?}", e);

        // Second attempt: wl-copy
        if let Err(e) = Command::new("wl-copy")
            .arg(content)
            .status() {
            eprintln!("wl-copy failed: {}", e);
            return Err("Both clipboard methods failed".into());
        }
    }

    // Give the clipboard system time to process
    thread::sleep(Duration::from_millis(100));

    // Verify the clipboard content
    if let Ok(output) = Command::new("wl-paste").output() {
        if output.status.success() {
            let pasted = String::from_utf8_lossy(&output.stdout);
            if pasted == content {
                println!("Clipboard content set and verified successfully");
                return Ok(());
            }
        }
    }

    Err("Failed to verify clipboard content".into())
}

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
            default_excludes.push(".git/".to_string());
            default_excludes.push("./.git/".to_string());
            default_excludes.push("node_modules/".to_string());
            default_excludes.push("package-lock.json".to_string());
            default_excludes.push(".css".to_string());
            default_excludes.push(".vscode/".to_string());
            default_excludes.push(".idea/".to_string());
            default_excludes.push("dist/".to_string());
            default_excludes.push("dist/".to_string());
            default_excludes.push(".env".to_string());
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

    // Set and verify clipboard content
    set_clipboard(&clipboard_content)?;

    Ok(())
}
