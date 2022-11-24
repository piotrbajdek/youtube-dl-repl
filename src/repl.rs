// YOUTUBE-DL-REPL VERSION 1.0.0-ALPHA.1 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE REPL

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::similar_names, clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::io;
use std::process::Command;

// HELP

fn help(reset: &str, blue_underlined: &str, cyan: &str, yellow: &str) {
    println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " source and documentation:");
    println!();
    println!("{}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/youtube-dl-repl" + reset);
    println!();
    println!("{}", cyan.to_owned() + "youtube-dl-repl" + reset + " help:");
    println!();
    println!("Provide youtube-dl options as youtube-dl-repl command-line arguments");
    println!("Next, you will be asked for the URL in the interactive shell mode...");
    println!();
    println!("Example: {}", yellow.to_owned() + "youtube-dl-repl -f 140" + reset);
    println!();
    println!("{}", cyan.to_owned() + "youtube-dl" + reset + " help:");
    println!();
    Command::new("youtube-dl")
        .arg("--help")
        .status()
        .unwrap();
}

// LICENSE

fn license(reset: &str, yellow: &str) {
    println!("{}", yellow.to_owned() + "MIT License" + reset);
    println!();
    println!("Copyright © 2022 Piotr Bajdek");
    println!();
    println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
    println!();
    println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
    println!();
    println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
}

// REPL

pub fn interactive(reset: &str, blue_underlined: &str, red: &str, cyan: &str, yellow: &str) {
    let args: Vec<String> = env::args().collect();
    let arg_cnt = args.len();

    if arg_cnt == 1 {
        println!("{}", red.to_owned() + "Warning: no youtube-dl options provided!" + reset);
        println!();
        println!("Provide youtube-dl options as youtube-dl-repl command-line arguments");
        println!("Next, you will be asked for the URL in the interactive shell mode...");
        println!();
        println!("Example: {}", yellow.to_owned() + "youtube-dl-repl -f 140" + reset);
        println!();
    }

    println!("Type {}", cyan.to_owned() + "quit" + reset + " or " + cyan + "exit" + reset + " to close the REPL");
    println!("Type {}", cyan.to_owned() + "help" + reset + " to see the usage and exit");
    println!("Type {}", cyan.to_owned() + "license" + reset + " to display the license");
    println!();
    println!("Insert your URL:");

    if arg_cnt == 1 {
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 2 {
        let arg1 = args.get(1).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 3 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 4 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 5 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 6 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 7 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 8 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 9 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 10 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        let arg9 = args.get(9).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .arg(arg9)
                .status()
                .unwrap();
        }
    }

    if arg_cnt == 11 {
        let arg1 = args.get(1).unwrap();
        let arg2 = args.get(2).unwrap();
        let arg3 = args.get(3).unwrap();
        let arg4 = args.get(4).unwrap();
        let arg5 = args.get(5).unwrap();
        let arg6 = args.get(6).unwrap();
        let arg7 = args.get(7).unwrap();
        let arg8 = args.get(8).unwrap();
        let arg9 = args.get(9).unwrap();
        let arg10 = args.get(10).unwrap();
        loop {
            println!("{}", cyan);
            let mut input_repl = String::new();
            io::stdin()
                .read_line(&mut input_repl)
                .expect(&(red.to_owned() + "Unable to read entered data" + reset));
            println!("{}", reset);
            let url_to_dl: &str = input_repl.trim();
            if url_to_dl == "quit" || url_to_dl == "exit" {
                return;
            } else if url_to_dl == "help" {
                help(reset, blue_underlined, cyan, yellow);
                return;
            } else if url_to_dl == "license" {
                license(reset, yellow);
                return;
            }
            Command::new("youtube-dl")
                .arg(url_to_dl)
                .arg(arg1)
                .arg(arg2)
                .arg(arg3)
                .arg(arg4)
                .arg(arg5)
                .arg(arg6)
                .arg(arg7)
                .arg(arg8)
                .arg(arg9)
                .arg(arg10)
                .status()
                .unwrap();
        }
    }
}