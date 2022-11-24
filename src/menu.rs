// YOUTUBE-DL-REPL VERSION 1.0.0 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

// IMPORTS

use std::env;
use std::process::exit;
use std::process::Command;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, red: &str, cyan: &str, yellow: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " source and documentation:");
            println!();
            println!("{}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/youtube-dl-repl" + reset);
            println!();
            println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " help:");
            println!();
            println!("Provide youtube-dl options as youtube-dl-repl command-line arguments");
            println!("Next, you will be asked for the URL in the interactive shell mode...");
            println!();
            println!("Example: {}", yellow.to_owned() + "youtube-dl-repl -f 140" + reset + "        [" + red + "NOTE" + reset + ": do not insert any URL]");
            println!();
            println!("{}", cyan.to_owned() + "youtube-dl" + reset + " help:");
            println!();
            Command::new("youtube-dl")
                .arg("--help")
                .status()
                .unwrap();
            exit(0);
        }

        // VERSION 1

        if argument == "-v" {
            println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " version: " + yellow + "1.0.0" + reset);
            println!("2022.11.24");
            println!();
            println!("{}", cyan.to_owned() + "youtube-dl" + reset + " version:");
            Command::new("youtube-dl")
                .arg("-v")
                .status()
                .unwrap();
            exit(0);
        }

        // VERSION 2

        if argument == "--version" {
            println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " version: " + yellow + "1.0.0" + reset);
            println!("2022.11.24");
            println!();
            println!("{}", cyan.to_owned() + "youtube-dl" + reset + " version:");
            Command::new("youtube-dl")
                .arg("--version")
                .status()
                .unwrap();
            exit(0);
        }
    }
}
