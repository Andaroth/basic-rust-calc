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