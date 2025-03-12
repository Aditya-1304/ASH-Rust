# ASH Shell

ASH Shell is a simple, custom command-line shell written in Rust. This shell supports common commands such as `cd`, `ls`, `cat`, `mkdir`, `touch`, `rm`, `cp`, `mv`, `grep`, `pwd`, `echo`, and `date`.

## Features

- **Custom Error Handling:** Uses `thiserror` for clean error messages
- **Command History:** Integrated with `rustyline` for a better user experience
- **Signal Handling:** Gracefully handles interrupts with `ctrlc`
- **Linux Support:** Built specifically for Linux environments

## Installation

### For Linux Users

1. **Download the Binary:**

   Go to the [Releases](https://github.com/Aditya-1304/ASH-Rust/releases) section of this repository and download the file `ash-shell-linux`.

2. **Make the Binary Executable:**

   Open a terminal and navigate to where you downloaded the file, then first extract the tarball and execute:
   ```bash
   tar -xzvf ash-shell-v0.1.0-linux.tar.gz
   chmod +x ash-shell-linux
   ```

3. **Run the Shell:**

   Start the shell by running:
   ```bash
   ./ash-shell-linux
   ```

## Building from Source

If you prefer to build the shell from source on Linux:

1. Ensure you have Rust and Cargo installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone https://github.com/Aditya-1304/ASH-Rust.git
   ```

3. Build the project:
   ```bash
   cd ASH-Rust
   cargo build --release
   ```

4. Run the shell:
   ```bash
   ./target/release/ash-shell
   ```

## Supported Commands

- `cd [directory]` - Change directory
- `ls [directory]` - List directory contents
- `cat [file]` - Display file contents
- `mkdir [directory]` - Create a new directory
- `touch [file]` - Create a new file
- `rm [file/directory]` - Remove a file or directory
- `cp [source] [destination]` - Copy files
- `mv [source] [destination]` - Move files
- `grep [pattern] [file]` - Search file contents
- `pwd` - Print working directory
- `echo [text]` - Display text
- `date` - Display current date and time
- `exit` - Exit the shell

