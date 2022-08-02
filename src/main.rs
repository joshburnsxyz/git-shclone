use clap::{arg,command};
use std::process::{Command,exit};
use std::io::{self, Write};

fn main() {
    let matches = command!()
        .arg(arg!([REPO]))
        .arg(arg!([DEST]).default_value(&format!("./")))
        .arg(arg!(-h --host [HOST], "One of: github, gitlab"]))
        .get_matches();
    
    let repo_url = format!("git@github.com:{}", matches.get_one::<String>("REPO").unwrap());
    let dest_path = format!("./{}", matches.get_one::<String>("REPO").unwrap().split("/").next().unwrap());
    let _git_host = matches.get_one::<String>("HOST").unwrap();

    let output = Command::new("git")
        .args(["clone", &repo_url, &dest_path])
        .output()
        .expect("fail");

    // Write command output to respective IO streams
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // Exit with code zero
    exit(0);
}
