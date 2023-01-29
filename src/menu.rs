// YOUTUBE-DL-REPL VERSION 1.0.1 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

// IMPORTS

use std::env;
use std::process::exit;
use std::process::Command;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", violet.to_owned() + "youtube-dl-repl" + reset + "'" + grey + "s source and documentation" + reset + ":");
            println!();
            println!("{}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/youtube-dl-repl" + reset);
            println!();
            println!("{}", violet.to_owned() + "youtube-dl-repl" + reset + "'" + grey + "s help" + reset + ":");
            println!();
            println!("Provide youtube-dl options as youtube-dl-repl command-line arguments");
            println!("Next, you will be asked for the URL in the interactive shell mode...");
            println!();
            println!("{}", grey.to_owned() + "Example" + reset + ": " + yellow + "youtube-dl-repl -f 140" + reset + "        [" + red + "NOTE" + reset + ": do not insert any URL]");
            println!();
            println!("{}", violet.to_owned() + "youtube-dl" + reset + "'" + grey + "s help" + reset + ":");
            println!();
            Command::new("youtube-dl")
                .arg("--help")
                .status()
                .unwrap();
            exit(0);
        }

        // VERSION 1

        if argument == "-v" {
            println!("{}", violet.to_owned() + "youtube-dl-repl" + reset + "'" + grey + "s version" + reset + ": " + yellow + "1.0.1" + reset);
            println!("2023.01.29");
            println!();
            println!("{}", violet.to_owned() + "youtube-dl" + reset + "'" + grey + "s version" + reset + ":");
            Command::new("youtube-dl")
                .arg("-v")
                .status()
                .unwrap();
            exit(0);
        }

        // VERSION 2

        if argument == "--version" {
            println!("{}", violet.to_owned() + "youtube-dl-repl" + reset + "'" + grey + "s version" + reset + ": " + yellow + "1.0.1" + reset);
            println!("2023.01.29");
            println!();
            println!("{}", violet.to_owned() + "youtube-dl" + reset + "'" + grey + "s version" + reset + ":");
            Command::new("youtube-dl")
                .arg("--version")
                .status()
                .unwrap();
            exit(0);
        }
    }
}
