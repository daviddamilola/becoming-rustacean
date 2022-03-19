use std::io; //io library from the standard library std
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input your guess.");
        // declare mutable variable guess with a type of string
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess) // read the input into the string guess
            .expect("Failed to read line"); // catch error
    
        //convert string input to integer
    
        let guess: u32 = match guess.trim().parse() {// shadow initial guess variable, parse the input to an integer of type u32
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("you guessed: {}", guess); //cest finni {} is a place holder for guess
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
}
