use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

/// cat
/// Redirects output from a file to either stdout or a user-specified file
/// # Arguments
/// ```args: Vec<&str>``` - command-line arguments provided to ```cat```
pub fn cat(args: Vec<&str>) {
    let i: &mut usize = &mut 0;
    let mut files_r: Vec<&str> = Vec::new();
    while (*i < args.len()) && (args[*i] != ">") {
        files_r.push(args[*i]);
        *i += 1;
    }

    if files_r.is_empty() {
        panic!("No read files provided!");
    }
    
    if *i != args.len() {
        if (*i + 1) != (args.len() - 1) {
            panic!("Cannot write to more than 1 file!");
        } else {
            *i += 1;
            let file_w: &str = args[*i];
            let mut file_write = File::create(file_w).unwrap();
            for f in files_r {
                let contents = fs::read_to_string(f).expect("Cannot read file!");
                file_write.write_all(contents.as_bytes()).unwrap();
            }    
        }
    } else {
        for f in files_r {
            let contents = fs::read_to_string(f).expect("Cannot read file!");
            print!("{}", contents);
        }
    }
}

/// echo
/// Prints out the user's input on the interface
/// # Arguments
/// ```args: Vec<&str>``` - vector representing space-separated inputs from user
pub fn echo(args: Vec<&str>) {
    let mut out: String = String::new();
    for input in args {
        out += input;
        out += " ";
    }
    if out.len() > 1 {
        out = out[..out.len() - 1].to_string();
    }
    println!("{}", out);
}

/// ls
/// Prints out user's current working directory by calling Rust's native
/// ```env::current_dir``` function
pub fn ls() {
    if let Some(current_dir) = env::current_dir().unwrap().to_str() {
        println!("{}", current_dir);
    }
}