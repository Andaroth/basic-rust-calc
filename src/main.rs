use std::io;

fn print_menu() {
    println!("AB calc, choisissez");
    println!("[1] Addition");
    println!("[2] Soustraction");
    println!("[3] Division");
    println!("[4] Reste");
}

fn main() {
    let input = &mut String::new();
    let input_a = &mut String::new();
    let input_b = &mut String::new();
    loop {
        print_menu();
        input.clear();
        io::stdin().read_line(input).expect("Failed to read line");
        let select: i32 = input.trim().parse().expect("Entrez un nombre!");
        if select < 1 || select > 4 {
            println!("Choisissez une option valide (de 1 à 4)");
        } else {
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

            let mut res: f64 = 0.0;
            if select == 1 { res = a + b; }
            if select == 2 { res = a - b; }
            if select == 3 { res = a / b; }
            if select == 4 { res = a % b; }

            println!("Résultat: {}", res);
        }
    }
}