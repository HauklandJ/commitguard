use clap::{App, Arg};
use regex::Regex;
use std::{fs, io::{self, BufRead}, os::unix::fs::PermissionsExt, path::Path, process};

fn main() {
    let matches = App::new("guard")
        .version("1.0")
        .author("Johannes Haukland")
        .about("Manages git hooks for conventional commits")
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .help("Prints version number"))
        .arg(Arg::new("init")
            .short('i')
            .long("init")
            .help("Sets up git with conventional commits"))
        .arg(Arg::new("remove")
            .short('r')
            .long("remove")
            .help("Removes conventional commit validation"))
        .arg(Arg::new("check-commit-msg")
            .long("check-commit-msg")
            .takes_value(true))
        .get_matches();

    if matches.is_present("version") {
        println!("version: 1.0");
    } else if matches.is_present("init") {
        if let Err(e) = setup_git_hook() {
            println!("Failed to add conventional commits to Git repository: {}", e);
        }
    } else if matches.is_present("remove") {
        if let Err(e) = remove_git_hook() {
            println!("Failed to remove conventional commits from Git repository: {}", e);
        }
    } else if let Some(commit_msg_path) = matches.value_of("check-commit-msg") {
        verify_commit_message(commit_msg_path);
    } else {
        println!("Error: No valid command found. Please provide a valid command.");
        std::process::exit(1);
    }
}

fn setup_git_hook() -> io::Result<()> {
    if !Path::new(".git").exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Cannot find .git folder"));
    }

    let hook_path = ".git/hooks/commit-msg";
    if Path::new(hook_path).exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "pre-commit file exists"));
    }

    let commit_content = "#!/bin/bash\nguard --check-commit-msg \"$1\"";
    fs::write(hook_path, commit_content.as_bytes())?;
    fs::set_permissions(hook_path, fs::Permissions::from_mode(0o777))?;

    Ok(())
}

fn remove_git_hook() -> io::Result<()> {
    let hook_path = ".git/hooks/commit-msg";
    if !Path::new(hook_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "pre-commit file doesn't exist. Is this the root git directory?"));
    }

    fs::remove_file(hook_path)
}

fn verify_commit_message(commit_msg_path: &str) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("This command must be run from within a Git commit hook directory\nTo setup, run 'guard -i' in a Git repository");
        process::exit(1);
    }

    let file = fs::File::open(commit_msg_path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let commit_message: String = reader.lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");

    let re = Regex::new(r"^(feat|fix|docs|style|refactor|perf|test|chore|merge)(\(.+\))?!?: .+").unwrap();
    if !re.is_match(&commit_message) {
        println!("Error: Invalid commit-message. Please follow the conventional commit format:");
        println!("    <type>[optional scope]: <description>");
        println!("    [optional body]");
        println!("    [optional footer(s)]");
        println!("Valid types include: feat, fix, docs, style, refactor, perf, test, chore");
        process::exit(1);
    }
}
