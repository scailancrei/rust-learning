use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero!");
    let secret_number = rand::rng().random_range(1..=6);

    loop {
        let mut guess: String = String::new();
        println!("Por favor, introduce un numero: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("Has ganado!");
                break;
            }
        }
    }

    println!("El numero secreto es: {secret_number}");
}
