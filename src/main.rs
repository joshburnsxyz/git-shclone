use clap::{arg,command};
use std::process::{Command,exit};
use std::io::{self, Write};

fn main() {
    let matches = command!()
        .args([
            arg!([REPO] "The github repo i.e username/repo"),
            arg!(-H --host [HOST] "One of: github, gitlab").default_value("github")
        ])
        .get_matches();

    let git_host = matches.get_one::<String>("host").unwrap();

    // Validate --host option and set repo_url accordingly
    let repo_url = match git_host.as_str() {
        "github" => format!("git@github.com:{}", matches.get_one::<String>("REPO").unwrap()),
        "gitlab" => format!("git@gitlab.com:{}", matches.get_one::<String>("REPO").unwrap()),
        _ => {
            eprintln!("Invalid git host, please use one of the following.");
            eprintln!("* github");
            eprintln!("* gitlab");
            exit(1);
        }
    };

    let output = Command::new("git").args(["clone", &repo_url]).output()
        .expect(&format!("git-shclone failed to clone {}", &repo_url));

    // Write command output to respective IO streams
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // Exit with code zero
    exit(0);
}
