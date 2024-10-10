use std::io;

fn print_menu() {
    println!("Choisissez une opération:");
    println!("[1] Addition");
    println!("[2] Soustraction");
    println!("[3] Division");
    println!("[4] Reste");
    println!("[5] Quitter")
}

fn main() {
    println!("=================\n==   AB calc   ==\n=================\n");
    let input = &mut String::new();
    let input_a = &mut String::new();
    let input_b = &mut String::new();
    loop {
        print_menu();
        input.clear();
        io::stdin().read_line(input).expect("Failed to read line");
        let select: usize = input.trim().parse().expect("Entrez un nombre!");
        if select < 1 || select > 5 {
            println!("Choisissez une option valide (de 1 à 4)");
        } else {
            if select == 5 { break; }
            // prompt a
            input_a.clear();
            println!("Choisissez un premier terme:");
            io::stdin().read_line(input_a).expect("Failed to read line");
            let a: f64 = input_a.trim().parse().expect("Entrez un nombre!");
            // prompt b
            input_b.clear();
            println!("Choisissez un second terme:");
            io::stdin().read_line(input_b).expect("Failed to read line");
            let b: f64 = input_b.trim().parse().expect("Entrez un nombre!");

            let symbol: &str = ["+","-","/","%"][select - 1];
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