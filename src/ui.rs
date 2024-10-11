use std::io;
use std::io::Write;

use crate::math::OPERATIONS;

fn header() { // styled header
    println!("\x1b[32m=========================================\n=                AB calc                =\n=========================================\x1b[0m\n");
}

fn menu() { // shows all option + quit
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
    io::stdin().read_line(&mut String::new()).expect("");
}

pub fn clear_screen() { print!("{}[2J", 27 as char); } // erase display

pub fn escape_line() { io::stdout().flush().unwrap(); } // escape 1 \n return

pub fn error(text: &str) { println!("\n\x1b[31m{}\x1b[0m\n", text); } // colored print