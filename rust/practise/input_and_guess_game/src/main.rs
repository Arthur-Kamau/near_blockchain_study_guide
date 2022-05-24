use rand::Rng;
use std::cmp::Ordering;
use std::io;
   // let mut new_string = String::new(); //Create an empty string
    // new_string.push_str("Hello");
    // new_string.push_str(" World!");
    
    // println!("{}", new_string); //Prints Hello World!

    //let mut new_string_2 = "hello";
   // new_string_2.push_str(" World!");

fn main() {    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // while true loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
