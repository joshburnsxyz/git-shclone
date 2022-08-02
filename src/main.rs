use clap::{arg,command};
use std::process::{Command,exit};
use std::io::{self, Write};

fn main() {
    let matches = command!()
        .arg(arg!([REPO]))
        .arg(arg!([DEST]).default_value(&format!("./")))
        .get_matches();
    let repo_url = format!("git@github.com:{}", matches.get_one::<String>("REPO").unwrap());
    let dest_path = format!("./{}", matches.get_one::<String>("REPO").unwrap().split("/").next().unwrap());
    let output = Command::new("git")
        .args(["clone", &repo_url, &dest_path])
        .output()
        .expect("fail");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    exit(0);
}
