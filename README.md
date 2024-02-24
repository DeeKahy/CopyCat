# ccat

`ccat` (Copy CAT) is a simple Rust utility designed to walk a project tree and copy all text readable files to the clipboard with file names and file locations. It's particularly useful for quickly sharing code snippets or file contents without manually opening and copying from each file.
Personally i use to to quicky give chatgpt the context of my application when i face some error i dont understand.

## Installation

To install `ccat`, follow these steps:

1. **Clone the repository:**

   ```sh
   git clone https://github.com/DeeKahy/CopyCat.git
   cd CopyCat
   ```
2. **Compile the project:**

   Ensure you have Rust and Cargo installed. If not, install them from [rustup](https://rustup.rs/).

   Then run:

   ```sh
   cargo build --release
   ```

3. **Place the binary in your PATH:**

   After compilation, the binary will be located at `./target/release/ccat`. Move this binary to a location in your PATH for easy access:

Linux/MacOS:
   ```sh
   cp ./target/release/ccat /usr/local/bin/ccat
   ```

   Adjust the destination path as necessary depending on your operating system and preferences.
   **If using windows the file is `called ccat.exe`**


## Usage

To use `ccat`, navigate to a directory and run:

```sh
ccat
```

This will copy the contents of all files in the current directory (and its subdirectories) to the clipboard, excluding binary and non-text files.

### Excluding Files, Folders, and File Types

`ccat` allows you to exclude specific files, folders, or entire file types by passing them as arguments. Here are some examples:

- **Exclude specific files or folders:**
  ```sh
  ccat exclude_this_file.txt another_file_to_exclude.txt
  ```


- **Exclude a whole file type:**

  To exclude all `.log` files, you can use:

  ```sh
  ccat '*.log'
  ```

  Note: Use quotes around the file type to ensure the shell does not expand the wildcard.

## .gitignore

Files and directories listed in `.gitignore` are not automatically excluded by `ccat`. You must manually specify them as arguments if you wish to exclude them.
