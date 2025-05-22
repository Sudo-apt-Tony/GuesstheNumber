use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number! Select your difficulty:");
    println!("1. 1-10");
    println!("2. 1-100");
    println!("3. 1-1000");

    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");

    let difficulty: u32 = difficulty
        .trim()
        .parse()
        .expect("Please choose a difficulty from the list provided, numbered 1-3.");

    //let diff_setting: [i32, 3] = [1, 2, 3]; // I don't remember why I needed this
    let sec_num: u32;

    match difficulty {
        1 => sec_num = rand::rng().random_range(1..=10),
        2 => sec_num = rand::rng().random_range(1..=100),
        3 => sec_num = rand::rng().random_range(1..=1000),
        _ => {
            println!("Please choose from the list provided, numbered 1-3");
            return;
        }
    }

    //let sec_num = rand::rng()
    //    .random_range(1..=100);


    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&sec_num){
            Ordering::Greater => println!("Too High! Try again!"),
            Ordering::Less => println!("Too low! Try again!"),
            Ordering::Equal => {
                println!("Correct! Winner!");
                break;
            },
        }
    }
}
