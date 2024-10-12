use std::io;
use std::str::FromStr;
use std::fmt::Debug;

use crate::ui::escape_line;
use crate::ui::error;

pub fn is_str_number<T: FromStr>(text: &str) -> bool { // verify if parsable string to number
    return text.trim().parse::<T>().is_ok();
}
pub fn str_to_number<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug { // conversion
    return text.trim().parse::<T>().expect("Parsing error");
}
pub fn read_value(print: &str) -> String {
    print!("\x1b[0m{}: \x1b[35m", print);
    escape_line();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("Could not read line"); // prompt to ref
    return buffer;
}

pub fn ask_prompt(text: &str) -> f64 {
    let mut buffered: String = String::new(); // scoped buffer
    loop { // capture prompt until valid input
        buffered.clear();
        buffered = read_value(text); // prompt
        if !is_str_number::<f64>(&buffered) { error("Ce n'est pas un nombre valide"); } // warn
        else { break; } // is valid, proceed (exit current loop)
    }
    return str_to_number::<f64>(&buffered);
}