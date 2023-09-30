use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("**************************************\n");
    println!("Indovina il numero!");
    println!("È sicuramente un numero da 1 a 100\n");
    println!("**************************************\n");

    let secret_number= rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Scrivi il numero che pensi che sia...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Valore sbagliato! Riprova!");
        let guess:u32= match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Troppo piccolo, riprova!"),
            Ordering::Greater=> println!("Troppo grande, riprova!"),
            Ordering::Equal => {
                println!("Giusto, DAJEEEEEEEEEEEEE");
                break;
            }
        }
    }
}
