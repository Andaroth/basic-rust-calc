mod utils;
use crate::utils::{is_str_number,str_to_number,read_value,ask_prompt};

mod math;
use crate::math::OPERATIONS;

mod ui;
use crate::ui::{welcome,pause,clear_screen,fatal_error,printc};

fn main() {
    welcome(); // banner + menu
    let mut input: String = String::new(); // global buffer
    loop {
        input.clear();
        input = read_value("Choisissez une opération");
        if !is_str_number::<i64>(&input) { // error handling
            fatal_error("Saisie invalide, veuillez recommencer");
            continue; // skip current turn to prompt again
        }
        let select: usize = str_to_number::<i64>(&input) as usize; // usize to select operations by index
        if select < 1 || select > OPERATIONS.len() + 1 { // not existing option
            fatal_error(&format!("Choisissez une option valide (de 1 à {})", OPERATIONS.len() + 1));
        } else { // constrained to existing options
            if select == OPERATIONS.len() + 1 { // "quit" option
                printc("\nBye o/\n", "red");
                break; // kill loop (end program)
            }
            clear_screen();
            printc(&format!("\n[ {} ]\n", OPERATIONS[select - 1].0.to_uppercase()), "blue"); // display selected operation

            let a: f64 = ask_prompt("Choisissez le premier terme"); // prompt a
            let b: f64 = ask_prompt("Choisissez le second terme"); // prompt b

            let (name, operation, symbol) = OPERATIONS[select - 1]; // extract selected operation details
            let ans: f64 = operation(a,b); // execute operation function

            printc(&format!("\n{}: {} {} {} = \x1b[33m{}\n", name, a, symbol, b, ans), "green"); // show answer
            pause(); // ask to press enter

            welcome(); // banner + menu
        }
    }
}