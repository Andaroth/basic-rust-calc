use std::io;
use std::str::FromStr;
use std::fmt::Debug;

use crate::ui::escape_line;

pub fn is_str_number<T: FromStr>(text: &str) -> bool { // verify if parsable string to number
    return text.trim().parse::<T>().is_ok();
}
pub fn str_to_number<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug { // conversion
    return text.trim().parse::<T>().expect("Parsing error");
}
pub fn read_value(print: &str) -> String {
    print!("{}: ", print);
    escape_line();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("Could not read line"); // prompt to ref
    return buffer;
}