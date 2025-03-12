use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::exit;
use chrono::Local;
use dirs;
use ctrlc;
use rustyline::{Editor, error::ReadlineError};
use rustyline::history::{FileHistory, History};
use thiserror::Error;


#[derive(Error, Debug)]
enum ShellError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Missing Arguments: {0}")]
    MissingArguments(&'static str),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Is a directory: {0}")]
    IsDirectory(String),
}

type ShellResult<T> = Result<T, ShellError>;

fn main() {
    println!("ASH Shell - Aditya's Shell in Rust");

    let history_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ash_history");

    let mut rl = Editor::<(), FileHistory>::new().unwrap();
    if rl.load_history(&history_path).is_err() {
        eprintln!("No previous history found");
    }

    ctrlc::set_handler(move || {
        println!("\nType 'exit' to quit or use history to view commands");
    }).expect("Error setting Ctrl-C handler");
    
    loop {
        match print_prompt(&mut rl) {
            Ok(input) => {
                if input.is_empty() {
                    continue;
                }
                
                // Add to history
                let _ = rl.add_history_entry(&input);
                
                let (command, args) = parse_input(&input);
                if let Err(e) = execute_command(command, &args, &mut rl) {
                    handle_error(e, command, &args);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                eprintln!("Readline error: {}", err);
                break;
            }
        }
    }

    
    
    rl.save_history(&history_path)
        .unwrap_or_else(|e| eprintln!("Failed to save history: {}", e));
}

fn handle_error(error: ShellError, command: &str, _args: &[&str]) {
    match error {
        ShellError::Io(e) => {
            eprintln!("Error in {}: {}", command, e);
            if e.kind() == io::ErrorKind::PermissionDenied {
                eprintln!("Try running with elevated privileges");
            }
        }
        ShellError::InvalidArgument(msg) => {
            eprintln!("Invalid argument: {}", msg);
            eprintln!("Usage: {}", get_command_usage(command));
        }
        ShellError::MissingArguments(arg) => {
            eprintln!("Missing required argument: {}", arg);
            eprintln!("Usage: {}", get_command_usage(command));
        }
        ShellError::FileNotFound(path) => {
            eprintln!("File not found: {}", path);
            eprintln!("Check the path and try again");
        }
        ShellError::IsDirectory(path) => {
            eprintln!("Is a directory: {}", path);
            eprintln!("Did you mean to use a file instead?");
        }
        e => eprintln!("{}", e),
    }
}

fn get_command_usage(command: &str) -> &'static str {
    match command {
        "cd" => "cd [directory]",
        "ls" => "ls [directory]",
        "cat" => "cat <file>",
        "mkdir" => "mkdir <directory>",
        "touch" => "touch <file>",
        "cp" => "cp <source> <destination>",
        "mv" => "mv <source> <destination>",
        "rm" => "rm <file> [-r for directories]",
        "grep" => "grep <pattern> <file>",
        _ => "",
    }
}

// Helper functions
fn print_prompt(rl: &mut Editor<(), FileHistory>) -> Result<String, ReadlineError> {
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .display()
        .to_string();
    
    let prompt = format!("ASH$ {} > ", current_dir);
    rl.readline(&prompt)
}

fn _read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim_end().to_string()
}

fn parse_input(input: &str) -> (&str, Vec<&str>) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();
    (command, args)
}

fn execute_command(command: &str, args: &[&str], rl: &mut Editor<(), FileHistory>) -> ShellResult<()> {
    match command {
        "" => Ok(()),
        "exit" => exit(0),
        "cd" => cd(args),
        "help" => help(),
        "ls" => ls(args),
        "cat" => cat(args),
        "mkdir" => mkdir(args),
        "touch" => touch(args),
        "rm" => rm(args),
        "cp" => cp(args),
        "mv" => mv(args),
        "grep" => grep(args),
        "pwd" => pwd(),
        "echo" => echo(args),
        "date" => date(),
        "history" => show_history(rl),
        _ => Err(ShellError::CommandNotFound(command.to_string())),
    }
}

// Command implementations
fn cd(args: &[&str]) -> ShellResult<()> {
    let path = args.first().unwrap_or(&"");
    let path = if path.is_empty() {
        env::var("HOME").map_err(|_| ShellError::InvalidArgument("Home directory not found".into()))?
    } else {
        path.to_string()
    };

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(ShellError::FileNotFound(path));
    }
    
    env::set_current_dir(&path_buf)?;
    Ok(())
}

fn help() -> ShellResult<()> {
    println!("Implemented commands:");
    println!("  exit          - Exit the shell");
    println!("  cd [dir]      - Change directory");
    println!("  ls [path]     - List directory contents");
    println!("  cat <file>    - Display file content");
    println!("  mkdir <dir>   - Create directory");
    println!("  touch <file>  - Create empty file");
    println!("  rm <path>     - Remove file/directory");
    println!("  cp <src> <dst> - Copy file");
    println!("  mv <src> <dst> - Move/rename file");
    println!("  grep <pattern> <file> - Search text");
    println!("  pwd           - Print working directory");
    println!("  echo <text>   - Display message");
    println!("  date          - Show current date/time");
    println!("  help          - Show this help");
    println!("  history       - Show command history");
    Ok(())
}

fn ls(args: &[&str]) -> ShellResult<()> {
    let path = args.first().unwrap_or(&".");
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let fname = entry.file_name().into_string()
            .map_err(|_| ShellError::InvalidArgument("Invalid filename".into()))?;
        print!("{}  ", fname);
    }
    println!();
    Ok(())
}

fn cat(args: &[&str]) -> ShellResult<()> {
    if args.is_empty() {
        return Err(ShellError::MissingArguments("file"));
    }
    
    for file in args {
        let metadata = fs::metadata(file)?;
        if metadata.is_dir() {
            return Err(ShellError::IsDirectory(file.to_string()));
        }
        
        let content = fs::read_to_string(file)?;
        print!("{}", content);
    }
    Ok(())
}

fn mkdir(args: &[&str]) -> ShellResult<()> {
    if args.is_empty() {
        return Err(ShellError::MissingArguments("directory name"));
    }
    
    for dir in args {
        fs::create_dir(dir)?;
    }
    Ok(())
}

fn touch(args: &[&str]) -> ShellResult<()> {
    if args.is_empty() {
        return Err(ShellError::MissingArguments("file name"));
    }
    
    for file in args {
        let _ = fs::File::create(file)?;
    }
    Ok(())
}

fn rm(args: &[&str]) -> ShellResult<()> {
    if args.is_empty() {
        return Err(ShellError::MissingArguments("file or directory"));
    }
    
    for path in args {
        if *path == "-r" {
            continue;
        }
        
        let metadata = fs::metadata(path)
            .map_err(|_| ShellError::FileNotFound(path.to_string()))?;
        
        if metadata.is_dir() {
            if args.contains(&"-r") {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_dir(path)?;
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn cp(args: &[&str]) -> ShellResult<()> {
    if args.len() < 2 {
        return Err(ShellError::MissingArguments("source and destination"));
    }
    
    let (src, dest) = (args[0], args[1]);
    
    // Check if source exists
    if !Path::new(src).exists() {
        return Err(ShellError::FileNotFound(src.to_string()));
    }
    
    fs::copy(src, dest)?;
    Ok(())
}

fn mv(args: &[&str]) -> ShellResult<()> {
    if args.len() < 2 {
        return Err(ShellError::MissingArguments("source and destination"));
    }
    
    let (src, dest) = (args[0], args[1]);
    
    // Check if source exists
    if !Path::new(src).exists() {
        return Err(ShellError::FileNotFound(src.to_string()));
    }
    
    fs::rename(src, dest)?;
    Ok(())
}

fn grep(args: &[&str]) -> ShellResult<()> {
    if args.len() < 2 {
        return Err(ShellError::MissingArguments("pattern and file"));
    }
    
    let (pattern, file) = (args[0], args[1]);
    
    if !Path::new(file).exists() {
        return Err(ShellError::FileNotFound(file.to_string()));
    }
    
    let file_handle = fs::File::open(file)?;
    let reader = io::BufReader::new(file_handle);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}:{}: {}", file, i+1, line);
        }
    }
    Ok(())
}

fn pwd() -> ShellResult<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn echo(args: &[&str]) -> ShellResult<()> {
    println!("{}", args.join(" "));
    Ok(())
}

fn date() -> ShellResult<()> {
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}

fn show_history(rl: &Editor<(), FileHistory>) -> ShellResult<()> {
    let history = rl.history();
    if history.is_empty() {
        println!("No command history available");
    } else {
        for (idx, entry) in history.iter().enumerate() {
            println!("{}: {}", idx + 1, entry);
        }
    }
    Ok(())
}

