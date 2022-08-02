use clap::{arg,command};
use std::process::{Command,exit};
use std::io::{self, Write};

fn main() {
    let matches = command!()
        .arg(arg!([REPO]))
        .arg(arg!([DEST]).default_value(&format!("./")))
        .arg(arg!(-h --host [HOST], "One of: github, gitlab").default_value(&format!("github"))))
        .get_matches();
    
    let repo_url;
    let dest_path = format!("./{}", matches.get_one::<String>("REPO").unwrap().split("/").next().unwrap());
    let _git_host = matches.get_one::<String>("HOST").unwrap();

    // Validate --host option and set repo_url accordingly
    match _git_host {
        String::from("github") => repo_url = format!("git@github.com:{}", matches.get_one::<String>("REPO").unwrap()),
        String::from("gitlab") => repo_url = format!("git@gitlab.com:{}", matches.get_one::<String>("REPO").unwrap()),
        _ => {
            eprintln!("Invalid git host, please check and try again.");
            exit(1);
        } 
    }

    let output = Command::new("git")
        .args(["clone", &repo_url, &dest_path])
        .output()
        .expect(format!("git-shclone failed to clone {} to {}", &repo_url, &dest_path));

    // Write command output to respective IO streams
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // Exit with code zero
    exit(0);
}
