mod utils;
use crate::utils::{is_str_number,str_to_number,read_value};

mod math;
use crate::math::OPERATIONS;

mod ui;
use crate::ui::{display_start,pause,clear_screen,escape_line,error};

fn main() {
    display_start(); // welcome
    let mut input: String = String::new(); // global buffer
    input.clear();
    loop {
        input = read_value("Choisissez une opération");
        if !is_str_number::<i32>(&input) { // error handling
            error("Saisie invalide, veuillez recommencer"); // throw warn in console
            continue; // prompt again
        }
        let select: usize = str_to_number::<usize>(&input); // usize to select operations by index
        if select < 1 || select > OPERATIONS.len() + 1 { // not existing option
            display_start(); // menu
            escape_line(); // escape \n
            // coercion & concat of a warning message:
            let mut warn = "Choisissez une option valide (de 1 à ".to_owned(); // static left side of str
            warn.push_str(&(OPERATIONS.len() + 1).to_string()); // calculated, to str
            warn.push_str(")"); // static right side of str
            error(&warn);
        } else { // constrained to existing options
            if select == OPERATIONS.len() + 1 { break; } // "quit" option, kill loop (end program)
            clear_screen();
            println!("\n\x1b[34m[ {} ]\x1b[0m\n", OPERATIONS[select - 1].0.to_uppercase()); // display selected operation

            fn ask_prompt(text: &str) -> f64 {
                let mut buffered: String = String::new(); // global buffer
                loop { // capture prompt until valid input
                    buffered.clear();
                    buffered = read_value(text); // prompt
                    if !is_str_number::<f64>(&buffered) { error("Ce n'est pas un nombre valide"); } // warn
                    else { break; } // is valid, proceed
                }
                return str_to_number::<f64>(&buffered);
            }

            let a: f64 = ask_prompt("Choisissez le premier terme"); // prompt a
            let b: f64 = ask_prompt("Choisissez le premier terme"); // prompt b

            let (name, operation, symbol) = OPERATIONS[select - 1]; // extract selected operation details
            let ans: f64 = operation(a,b); // execute operation function

            println!("\n\x1b[32m{}: {} {} {} = \x1b[33m{}\x1b[0m \n", name, a, symbol, b, ans); // show answer
            pause(); // ask to press enter
            clear_screen();
            display_start(); // menu
        }
    }
}