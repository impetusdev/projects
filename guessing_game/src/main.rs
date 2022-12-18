use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your number");

    let mut guess = String::new(); // new is an associated function on String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
