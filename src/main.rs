use std::io;
use std::io::Write;
use std::str::FromStr;
use std::fmt::Debug;

static ERROR_MESSAGE: &str = "Une erreur s'est produite";
static INVALID_FLOAT: &str = "Ce n'est pas un nombre valide";
fn print_error(text: &str) { println!("\n\x1b[31m{}\x1b[0m\n", text); }

// implemented math functions
fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> f64 { a / b }
fn remainder(a: f64, b: f64) -> f64 { a % b }

static OPERATIONS: [(&'static str, fn(f64, f64) -> f64, char); 5] = [
    // Catalog operations (name, function, symbol)
    ("Addition", add, '+'),
    ("Soustraction", subtract, '-'),
    ("Multiplication", multiply, '*'),
    ("Division", divide, '/'),
    ("Reste Modulo", remainder, '%'),
];

fn print_menu() {
    // shows all option + quit
    for (i,m) in OPERATIONS.iter().enumerate() { println!("[{}] {}", i + 1, &m.0); }
    println!("[{}] Quitter\n", OPERATIONS.len() + 1);
}

fn is_str_number<T: FromStr>(text: &str) -> bool { return text.trim().parse::<T>().is_ok(); } // verify if parsable string to number
fn str_to_number<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug { return text.trim().parse::<T>().expect(&ERROR_MESSAGE); } // conversion

fn read_value(print: &str) -> String {
    print!("{}: ", print);
    io::stdout().flush().unwrap(); // prompt on 1 line
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect(ERROR_MESSAGE); // prompt to ref
    return buffer;
}

fn main() {
    println!("\x1b[32m=================\n=    AB calc    =\n=================\x1b[0m\n"); // styled header
    loop {
        print_menu();
        let input = read_value("Choisissez une opération");
        if !is_str_number::<i32>(&input) { // error handling
            print_error("Saisie invalide, veuillez recommencer");
            continue; // restart loop
        }
        let select: usize = str_to_number::<usize>(&input); // usize to select operations by index
        if select < 1 || select > OPERATIONS.len() + 1 { // not existing option
            // coercion & concat of a warning message:
            let mut warn = "Choisissez une option valide (de 1 à ".to_owned(); // static left side of str
            warn.push_str(&(OPERATIONS.len() + 1).to_string()); // calculated, to str
            warn.push_str(")"); // static right side of str
            print_error(&warn);
        } else { // constrained to existing options
            if select == OPERATIONS.len() + 1 { break; } // "quit" option, kill loop (end program)
            println!("\n\x1b[34m<<{}>>\x1b[0m\n", OPERATIONS[select - 1].0); // name of selected operation
            
            let input = read_value("Choisissez le premier terme"); // prompt a
            if !is_str_number::<f64>(&input) { // error handling
                print_error(INVALID_FLOAT);
                continue; // restart loop
            }
            let a: f64 = str_to_number::<f64>(&input); // save prompt a to stack
            
            let input = read_value("Choisissez le second terme"); // prompt b
            if !is_str_number::<f64>(&input) { // error handling
                print_error(INVALID_FLOAT);
                continue; // restart loop
            }
            let b: f64 = str_to_number::<f64>(&input); // save prompt b to stack

            let (name, operation, symbol) = OPERATIONS[select - 1]; // extract selected operation details
            let ans: f64 = operation(a,b); // execute operation function

            println!("\n\x1b[32m{}: {} {} {} = \x1b[33m{}\x1b[0m \n", name, a, symbol, b, ans); // show answer
        }
    }
}