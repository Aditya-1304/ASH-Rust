use std::env;
use std::fs;
use std::io::{self,BufRead, Write};
use std::path::Path;
use std::process::exit;
use chrono::Local;

fn main() {
    println!("ASH Shell - Advanced Shell in Rust");
    loop {
        print_prompt();
        let input = read_input();
        let (command, args) = parse_input(&input);
        
        match command {
            "" => continue,
            "exit" => exit(0),
            "cd" => cd(&args),
            "help" => help(),
            "ls" => ls(&args),
            "cat" => cat(&args),
            "mkdir" => mkdir(&args),
            "touch" => touch(&args),
            "rm" => rm(&args),
            "cp" => cp(&args),
            "mv" => mv(&args),
            "grep" => grep(&args),
            "pwd" => pwd(),
            "echo" => echo(&args),
            "date" => date(),
            _ => println!("Command not found: {}", command),
        }
    }
}

// Helper functions (same as before)
fn print_prompt() { 
    let current_dir = env::current_dir().unwrap();
    print!("ASH$ {} > ", current_dir.display());
    io::stdout().flush().unwrap();
 }
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim_end().to_string()
}
fn parse_input(input: &str) -> (&str, Vec<&str>) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        ("", vec![])
    } else {
        (tokens[0], tokens[1..].to_vec())
    }
}

// Command implementations
fn cd(args: &[&str]) {
    let path = args.first().unwrap_or(&"");
    let path = if path.is_empty() { 
        env::var("HOME").unwrap() 
    } else { 
        path.to_string() 
    };
    
    if let Err(e) = env::set_current_dir(Path::new(&path)) {
        eprintln!("cd: {}", e);
    }
}

fn help() {
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
}

fn ls(args: &[&str]) {
    let path = args.first().unwrap_or(&".");
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let fname = entry.file_name().into_string().unwrap();
                    print!("{}  ", fname);
                }
            }
            println!();
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

fn cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: missing file operand");
        return;
    }
    
    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("cat: {}: {}", file, e),
        }
    }
}

fn mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }
    
    for dir in args {
        if let Err(e) = fs::create_dir(dir) {
            eprintln!("mkdir: {}: {}", dir, e);
        }
    }
}

fn touch(args: &[&str]) {
    if args.is_empty() {
        eprintln!("touch: missing file operand");
        return;
    }
    
    for file in args {
        if let Err(e) = fs::File::create(file) {
            eprintln!("touch: {}: {}", file, e);
        }
    }
}

fn rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    
    for path in args {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => {
                eprintln!("rm: {}: No such file or directory", path);
                continue;
            }
        };
        
        let result = if metadata.is_dir() {
            fs::remove_dir(path)
        } else {
            fs::remove_file(path)
        };
        
        if let Err(e) = result {
            eprintln!("rm: {}: {}", path, e);
        }
    }
}

fn cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("cp: missing destination file operand");
        return;
    }
    
    let (src, dest) = (args[0], args[1]);
    if let Err(e) = fs::copy(src, dest) {
        eprintln!("cp: {}: {}", src, e);
    }
}

fn mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("mv: missing destination file operand");
        return;
    }
    
    let (src, dest) = (args[0], args[1]);
    if let Err(e) = fs::rename(src, dest) {
        eprintln!("mv: {}: {}", src, e);
    }
}

fn grep(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("grep: missing pattern or file");
        return;
    }
    
    let (pattern, file) = (args[0], args[1]);
    match fs::File::open(file) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    if line.contains(pattern) {
                        println!("{}:{}: {}", args[1], i+1, line);
                    }
                }
            }
        }
        Err(e) => eprintln!("grep: {}: {}", file, e),
    }
}

fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn date() {
    let now = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}
