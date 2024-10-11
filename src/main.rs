mod utils;
use crate::utils::{is_str_number,str_to_number,read_value};

mod math;
use crate::math::OPERATIONS;

mod ui;
use crate::ui::{display_start,pause,clear_screen,escape_line,error,printc};

fn main() {
    display_start(); // welcome
    let mut input: String = String::new(); // global buffer
    loop {
        input.clear();
        input = read_value("Choisissez une opération");
        if !is_str_number::<i32>(&input) { // error handling
            error("Saisie invalide, veuillez recommencer"); // throw warn in console
            continue; // prompt again
        }
        let select: usize = str_to_number::<usize>(&input); // usize to select operations by index
        if select < 1 || select > OPERATIONS.len() + 1 { // not existing option
            display_start(); // menu
            escape_line(); // escape \n
            error(&format!("Choisissez une option valide (de 1 à {})", OPERATIONS.len() + 1));
        } else { // constrained to existing options
            if select == OPERATIONS.len() + 1 {// "quit" option
                printc("\nBye o/\n", "red");
                break; // kill loop (end program)
            }
            clear_screen();
            printc(&format!("\n[ {} ]\n", OPERATIONS[select - 1].0.to_uppercase()), "blue"); // display selected operation

            fn ask_prompt(text: &str) -> f64 {
                let mut buffered: String = String::new(); // scoped buffer
                loop { // capture prompt until valid input
                    buffered.clear();
                    buffered = read_value(text); // prompt
                    if !is_str_number::<f64>(&buffered) { error("Ce n'est pas un nombre valide"); } // warn
                    else { break; } // is valid, proceed
                }
                return str_to_number::<f64>(&buffered);
            }

            let a: f64 = ask_prompt("Choisissez le premier terme"); // prompt a
            let b: f64 = ask_prompt("Choisissez le second terme"); // prompt b

            let (name, operation, symbol) = OPERATIONS[select - 1]; // extract selected operation details
            let ans: f64 = operation(a,b); // execute operation function

            printc(&format!("\n{}: {} {} {} = \x1b[33m{}\n", name, a, symbol, b, ans), "green"); // show answer
            pause(); // ask to press enter
            
            clear_screen();
            display_start(); // menu
        }
    }
}