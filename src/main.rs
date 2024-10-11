mod utils;
use crate::utils::clear_screen;
use crate::utils::escape_line;
use crate::utils::pause;
use crate::utils::is_str_number;
use crate::utils::str_to_number;
use crate::utils::read_value;
use crate::utils::error;

mod math;
use crate::math::OPERATIONS;

mod ui;
use crate::ui::display_start;

fn main() {
    static INVALID_FLOAT: &str = "Ce n'est pas un nombre valide";
    display_start(); // welcome
    let mut input: String = String::new(); // global buffer
    input.clear();
    loop {
        input = read_value("Choisissez une opération");
        if !is_str_number::<i32>(&input) { // error handling
            error("Saisie invalide, veuillez recommencer");
            continue; // restart loop
        }
        clear_screen();
        let select: usize = str_to_number::<usize>(&input); // usize to select operations by index
        if select < 1 || select > OPERATIONS.len() + 1 { // not existing option
            display_start();
            escape_line();
            // coercion & concat of a warning message:
            let mut warn = "Choisissez une option valide (de 1 à ".to_owned(); // static left side of str
            warn.push_str(&(OPERATIONS.len() + 1).to_string()); // calculated, to str
            warn.push_str(")"); // static right side of str
            error(&warn);
        } else { // constrained to existing options
            if select == OPERATIONS.len() + 1 { break; } // "quit" option, kill loop (end program)
            println!("\n\x1b[34m[ {} ]\x1b[0m\n", OPERATIONS[select - 1].0.to_uppercase()); // name of selected operation

            loop {
                input.clear();
                input = read_value("Choisissez le premier terme"); // prompt a
                if !is_str_number::<f64>(&input) { error(INVALID_FLOAT); }
                else { break; }
            }
            let a: f64 = str_to_number::<f64>(&input); // save prompt a to stack
            
            loop {
                input.clear();
                input = read_value("Choisissez le second terme"); // prompt b
                if !is_str_number::<f64>(&input) { error(INVALID_FLOAT); }
                else { break; }
            }
            let b: f64 = str_to_number::<f64>(&input); // save prompt b to stack

            let (name, operation, symbol) = OPERATIONS[select - 1]; // extract selected operation details
            let ans: f64 = operation(a,b); // execute operation function

            println!("\n\x1b[32m{}: {} {} {} = \x1b[33m{}\x1b[0m \n", name, a, symbol, b, ans); // show answer
            pause();
            clear_screen();
            display_start();
        }
    }
}