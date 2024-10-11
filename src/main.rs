use std::io;
use std::str::FromStr;
use std::fmt::Debug;

type MenuType = [(&'static str, f64, char); 5];

static ERROR_MESSAGE: &str = "\nAn error occured\n";
fn print_error(text: &str) {
    println!("\n\x1b[31m{}\x1b[0m\n", text);
}

fn operations(a: f64, b: f64) -> MenuType {[
    ("Addition", a + b, '+'),
    ("Soustraction", a - b, '-'),
    ("Multiplication", a * b, '*'),
    ("Division", a / b, '/'),
    ("Reste", a % b, '%'),
]}

fn print_menu() {
    println!("Choisissez une opération:");
    for (i,m) in operations(0.0,0.0).iter().enumerate() {
        println!("[{}] {}", i + 1, &m.0);
    }
    println!("[{}] Quitter", operations(0.0,0.0).len() + 1);
}
fn menu() -> MenuType { operations(0.0,0.0) }

fn is_str_number<T: FromStr>(text: &str) -> bool { return text.trim().parse::<T>().is_ok(); }
fn str_to_number<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug { return text.trim().parse::<T>().expect(&ERROR_MESSAGE); }

fn main() {
    println!("=================\n==   AB calc   ==\n=================\n");
    let buffer = &mut String::new();
    loop {
        print_menu();
        buffer.clear();
        io::stdin().read_line(buffer).expect(ERROR_MESSAGE);
        if !is_str_number::<i32>(buffer) {
            print_error("Saisie invalide, veuillez recommencer");
            continue;
        }
        let select: usize = str_to_number::<usize>(buffer);
        if select < 1 || select > menu().len() + 1 {
            // coercion & concat of a warning message
            let mut warn = "Choisissez une option valide (de 1 à ".to_owned(); // static left side of str
            warn.push_str(&(menu().len() + 1).to_string()); // calculated, to str
            warn.push_str(")"); // static right side of str
            print_error(&warn);
        } else {
            if select == menu().len() + 1 { break; }
            println!("\n\x1b[34m<<{}>>\x1b[0m\n", operations(0.0,0.0)[select - 1].0);
            // prompt a
            buffer.clear();
            println!("Choisissez le premier terme:");
            io::stdin().read_line(buffer).expect(ERROR_MESSAGE);
            if !is_str_number::<f64>(buffer) {
                print_error("Ce terme n'est pas un nombre valide (float 64)");
                continue;
            }
            let a: f64 = str_to_number::<f64>(buffer);
            // prompt b
            buffer.clear();
            println!("Choisissez le second terme:");
            io::stdin().read_line(buffer).expect(ERROR_MESSAGE);
            if !is_str_number::<f64>(buffer) {
                print_error("Ce terme n'est pas un nombre valide (float 64)");
                continue;
            }
            let b: f64 = str_to_number::<f64>(buffer);

            let symbol: char = operations(a,b)[select - 1].2;
            let ans: f64 = operations(a,b)[select - 1].1;

            println!("\n\n\n{} {} {} = {} \n\n\n", a, symbol, b, ans);
        }
    }
}