use std::io;
use std::io::Write;
use std::str::FromStr;
use std::fmt::Debug;

type MenuType = [(&'static str, fn(f64, f64) -> f64, char); 5];

static ERROR_MESSAGE: &str = "Une erreur s'est produite";
static INVALID_FLOAT: &str = "Ce n'est pas un nombre valide";
fn print_error(text: &str) {
    println!("\n\x1b[31m{}\x1b[0m\n", text);
}

fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> f64 { a / b }
fn remainder(a: f64, b: f64) -> f64 { a % b }

static OPERATIONS: MenuType = [
    ("Addition", add, '+'),
    ("Soustraction", subtract, '-'),
    ("Multiplication", multiply, '*'),
    ("Division", divide, '/'),
    ("Reste Modulo", remainder, '%'),
];

fn print_menu() {
    for (i,m) in OPERATIONS.iter().enumerate() { println!("[{}] {}", i + 1, &m.0); }
    println!("[{}] Quitter", OPERATIONS.len() + 1);
}

fn is_str_number<T: FromStr>(text: &str) -> bool { return text.trim().parse::<T>().is_ok(); }
fn str_to_number<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug { return text.trim().parse::<T>().expect(&ERROR_MESSAGE); }

fn read_value(print: &str) -> String {
    print!("{}: ", print);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect(ERROR_MESSAGE);
    return buffer;
}

fn main() {
    println!("=================\n==   AB calc   ==\n=================\n");
    loop {
        print_menu();
        let input = read_value("Choisissez une opération");
        if !is_str_number::<i32>(&input) {
            print_error("Saisie invalide, veuillez recommencer");
            continue;
        }
        let select: usize = str_to_number::<usize>(&input);
        if select < 1 || select > OPERATIONS.len() + 1 {
            // coercion & concat of a warning message
            let mut warn = "Choisissez une option valide (de 1 à ".to_owned(); // static left side of str
            warn.push_str(&(OPERATIONS.len() + 1).to_string()); // calculated, to str
            warn.push_str(")"); // static right side of str
            print_error(&warn);
        } else {
            if select == OPERATIONS.len() + 1 { break; }
            println!("\n\x1b[34m<<{}>>\x1b[0m\n", OPERATIONS[select - 1].0);
            // prompt a
            let input = read_value("Choisissez le premier terme");
            if !is_str_number::<f64>(&input) {
                print_error(INVALID_FLOAT);
                continue;
            }
            let a: f64 = str_to_number::<f64>(&input);
            // prompt b
            let input = read_value("Choisissez le second terme");
            if !is_str_number::<f64>(&input) {
                print_error(INVALID_FLOAT);
                continue;
            }
            let b: f64 = str_to_number::<f64>(&input);

            let (_name, operation, symbol) = OPERATIONS[select - 1];
            let ans: f64 = operation(a,b);

            println!("\n\x1b[31m{} {} {} = {}\x1b[0m \n", a, symbol, b, ans);
        }
    }
}