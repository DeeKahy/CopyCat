use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    println!("Program started");

    let mut args: Vec<String> = env::args().skip(1).collect();

    // Find the gitignore file and add its contents to the blacklist
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
            Err(e) => eprintln!("Error reading .gitignore: {}", e),
        }
    }

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
                    clipboard_content.push_str(&format!("{:?}```{}```", path, contents));
                }
                Err(_) => continue, // Skip non-text files or errors
            }
        }
    }

    println!("Clipboard content length is: {}", clipboard_content.len());
    println!("First 100 characters of content: {:?}", clipboard_content.chars().take(100).collect::<String>());

    // Attempt to set clipboard content using cli-clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    match ctx.set_contents(clipboard_content.clone()) {
        Ok(_) => println!("Successfully set clipboard contents using cli-clipboard"),
        Err(e) => eprintln!("Failed to set clipboard contents using cli-clipboard: {:?}", e),
    }

    // Always try Wayland method
    println!("Attempting to use wl-copy...");
    let wl_copy_start = Instant::now();
    match Command::new("wl-copy")
        .arg(&clipboard_content)
        .stdin(Stdio::piped())
        .spawn() {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                if let Err(e) = stdin.write_all(clipboard_content.as_bytes()) {
                    eprintln!("Failed to write to wl-copy: {}", e);
                }
            }
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!("Successfully set clipboard contents using wl-copy");
                    } else {
                        eprintln!("wl-copy process exited with non-zero status: {:?}", status);
                    }
                }
                Err(e) => eprintln!("Error waiting for wl-copy: {}", e),
            }
        }
        Err(e) => eprintln!("Error executing wl-copy: {}", e),
    }
    println!("wl-copy operation took {:?}", wl_copy_start.elapsed());

    // Verify clipboard content
    println!("Verifying clipboard content...");
    let verify_start = Instant::now();
    match Command::new("wl-paste").output() {
        Ok(output) => {
            if output.status.success() {
                let pasted_content = String::from_utf8_lossy(&output.stdout);
                println!("Clipboard content (first 100 chars): {:?}", 
                    pasted_content.chars().take(100).collect::<String>());
                if pasted_content == clipboard_content {
                    println!("Clipboard content matches expected content.");
                } else {
                    println!("Clipboard content does not match expected content.");
                }
            } else {
                eprintln!("Failed to read clipboard contents using wl-paste");
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("Error executing wl-paste: {}", e),
    }
    println!("Verification operation took {:?}", verify_start.elapsed());

    println!("Program finished. Total execution time: {:?}", start_time.elapsed());
    Ok(())
}

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;
    String::from_utf8(contents).map_err(|_| std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Not a UTF-8 file",
    ))
}
