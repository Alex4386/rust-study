pub fn read_number()->i64 {
  let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Stdin I/O Failed");

    let parsed_input:i64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You didn't input the number, fallback to 0");
            0
        }
    };

    println!("The number you input is {}.", parsed_input);

    parsed_input
}