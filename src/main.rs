use std::cmp::Ordering;
use rand::{Rng, RngCore};

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut rand_num = rand::thread_rng().next_u32();
    println!("Hello, world with rand u32: {}!", rand_num);
    // gen_rage is upper bounds exclusive, that's why the "=50" thingy
    rand_num = rand::thread_rng().gen_range(1..=50);
    println!(
        "The double of a rand number [{}] is: {}",
        rand_num,
        double_an_int(rand_num)
    );

    print_something();

    let p1 = Person {
        name: String::from("Serj"),
        age: 35,
    };
    println!("-> created person: {}, age: {}", p1.name, p1.age);

    let mut guess = String::new();
    match std::io::stdin().read_line(&mut guess) {
        Ok(input_size) => {
            println!("entered input size: {}", input_size);
            let guess: u32 = guess.
                trim().
                parse().
                expect("Please type a number!");

            println!("You guessed: {}", guess);

            match guess.cmp(&rand_num) {
                Ordering::Less => println!("Too small, the num was: {0}!", rand_num),
                Ordering::Greater => println!("Too big, the num was: {rand_num}!"),
                Ordering::Equal => println!("You win!"),
            }
        }
        Err(_) => {
            println!("Failed to read line")
        }
    }

    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    //
    // println!("You guessed: {}", guess);
}

fn print_something() {
    println!("stupid function that only prints this");
}

fn double_an_int(an_int: u32) -> u32 {
    return an_int * 2;
}
