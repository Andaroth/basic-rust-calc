use std::io;

fn print_menu() {
    println!("Choisissez une opération:");
    println!("[1] Addition");
    println!("[2] Soustraction");
    println!("[3] Division");
    println!("[4] Reste");
    println!("[5] Quitter");
}


fn main() {
    println!("=================\n==   AB calc   ==\n=================\n");
    let buffer = &mut String::new();
    loop {
        print_menu();
        buffer.clear();
        io::stdin().read_line(buffer).expect("Failed to read line");
        if !buffer.trim().parse::<i32>().is_ok() {
            println!("Entrez un nombre!");
            continue;
        }
        let select: usize = buffer.trim().parse().expect("Entrez un nombre!");
        if select < 1 || select > 5 {
            println!("Choisissez une option valide (de 1 à 5)");
        } else {
            if select == 5 { break; }
            // prompt a
            buffer.clear();
            println!("Choisissez un premier terme:");
            io::stdin().read_line(buffer).expect("Failed to read line");
            if !buffer.trim().parse::<f64>().is_ok() {
                println!("Entrez un nombre!");
                continue;
            }
            let a: f64 = buffer.trim().parse().expect("Entrez un nombre!");
            // prompt b
            buffer.clear();
            println!("Choisissez un second terme:");
            io::stdin().read_line(buffer).expect("Failed to read line");
            if !buffer.trim().parse::<f64>().is_ok() {
                println!("Entrez un nombre!");
                continue;
            }
            let b: f64 = buffer.trim().parse().expect("Entrez un nombre!");

            let symbol: char = ['+','-','/','%'][select - 1];
            let ans: f64 = [
                a + b,
                a - b,
                a / b,
                a % b,
            ][select - 1];

            println!("\n\n\n{} {} {} = {} \n\n\n", a, symbol, b, ans);
        }
    }
}