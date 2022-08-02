use clap::{arg,command};
use std::process::{Command,exit};

fn main() {
    let matches = command!()
        .arg(arg!([REPO]))
        .arg(arg!([DEST]).default_value(&format!("./")))
        .get_matches();
    let repo_url = format!("git@github.com:{}", matches.get_one::<String>("REPO").unwrap());
    let dest_path = format!("./{}", matches.get_one::<String>("REPO").unwrap().split("/").next().unwrap());
    println!("{:#?}", Command::new("git")
        .args(["clone", &repo_url, &dest_path])
        .output()
        .expect("fail"));

    exit(0);
}
