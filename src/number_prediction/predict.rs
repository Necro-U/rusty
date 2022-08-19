use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Hello, world!");
    println!("Selamun Aleyküm");

    let sec_number = rand::thread_rng().gen_range(0, 100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line");
        let guess: u32 = guess.trim().parse().expect("Please send a number ");

        match guess.cmp(&sec_number) {
            Ordering::Equal => {
                println!("{}", "You are right".green());
                break;
            }
            Ordering::Greater => println!("{}", "try lower number".red()),
            Ordering::Less => println!("{}", "try bigger number".red()),
        }
        println!("your guess:{} , secret_num:{}", &guess, &sec_number);
    }
}
