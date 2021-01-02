use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    
    println!("Generating Numbers...");
    let random_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Enter your Input...");
    
        let mut user_guess = String::new();

        io::stdin().read_line(&mut user_guess)
            // expect will run when exceptions are occurred,
            // terminaling program.
            .expect("Readline fail");


        println!("Your Input: {}", user_guess);

        // error handling: 
        // just change expect -> match expression
        let user_guess_number: i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match user_guess_number.cmp(&random_number) {
            Ordering::Less => println!("UP!"),
            Ordering::Greater => println!("DOWN!"),
            Ordering::Equal => {
                println!("DINGDONG!");
                break;
            },
        }
    }


    
}
