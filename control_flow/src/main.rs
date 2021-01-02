use std::io;
use read_number;

fn main() {
    let parsed_input = read_number();
    
    if parsed_input > 50 {
        println!("Number is bigger than 50");
    }

    let fifty_below = if parsed_input >= 50 { false } else { true };
    println!("Is fifty_below? {}", fifty_below);

    let mut game_point = 0;

    let my_point = loop {
        game_point += 1;

        if game_point == 30 {
            break game_point * 5;
        }
    };

    println!("My point is {}", my_point);

    println!("Counting 1 to 10");

    for number in (1..11) {
        println!("{}", number);
    }

    for number in (1..11).rev() {
        println!("{}", number);
    }

    let sheep: [i64;5] = [10, 20, 30, 40, 50];

    for element in sheep.iter() {
        println!("{}", element);
    }

    





}
