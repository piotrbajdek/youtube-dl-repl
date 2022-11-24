// YOUTUBE-DL-REPL VERSION 1.0.0-ALPHA.1 / MIT LICENSE © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod menu;
pub mod repl;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let blue_underlined = "\x1b[34;4m";
    let red = "\x1b[31m";
    let cyan = "\x1b[36m";
    let yellow = "\x1b[93m";

    menu::documentation(reset, blue_underlined, cyan, yellow);
    repl::interactive(reset, blue_underlined, red, cyan, yellow);
}
