use std::io;
use std::io::Write;

use crate::math::OPERATIONS;

fn header() { // styled header
    printc("=========================================\n=             AB calc by Anda           =\n=========================================\n", "green");
}

fn menu() { // shows all options + quit
    for (i,m) in OPERATIONS.iter().enumerate() { println!("[{}] {}", i + 1, &m.0); }
    print!("[{}] Quitter\n", OPERATIONS.len() + 1);
}

pub fn display_start() {
    header(); // logo
    menu(); // options
}

pub fn pause() {
    print!("Appuyez sur Enter...");
    escape_line(); 
    io::stdin().read_line(&mut String::new()).expect(""); // fake pause (prompt into void)
}

pub fn clear_screen() { print!("{}[2J", 27 as char); } // erase display

pub fn escape_line() { io::stdout().flush().unwrap(); } // escape 1 \n return

pub fn printc(text: &str, color: &str) {
    let code = match color {
        "red" => "31m",
        "green" => "32m",
        "yellow" => "33m",
        "blue" => "34m",
        "cyan" => "36m",
        _ => "0m", // default no color (assumed fallback white)
    };
    println!("\n\x1b[{}{}\x1b[0m\n", code, text);
}

pub fn error(message: &str) { printc(&format!("\n{}\n", message), "red"); } // colored print
