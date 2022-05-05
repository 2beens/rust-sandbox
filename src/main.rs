use rand::{Rng, RngCore};
use std::cmp::Ordering;

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut rand_num = rand::thread_rng().next_u32();
    println!("Hello, world with rand u32: {}!", rand_num);
    // gen_rage is upper bounds exclusive, that's why the "=10" thingy
    rand_num = rand::thread_rng().gen_range(1..=10);
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

    loop {
        rand_num = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess) {
            Ok(input_size) => {
                guess = String::from(guess.trim());
                if guess == "quit" {
                    break;
                }

                println!("entered input {guess}, size: {input_size}");
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("NaN entered");
                        continue;
                    }
                };

                println!("You guessed: {}", guess);

                match guess.cmp(&rand_num) {
                    Ordering::Less => println!("Too small, the num was: {0}!", rand_num),
                    Ordering::Greater => println!("Too big, the num was: {rand_num}!"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
            Err(_) => {
                println!("Failed to read line")
            }
        }
    }

    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    //
    // println!("You guessed: {}", guess);
}

fn print_something() {
    // A new scope block created with curly brackets is an expression, for example:
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let mut counter: u8 = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // 5 and 10 are statements, so their scopes actually return that value
    let ternary_op_example = if counter == 5 { 5 } else { 10 };
    println!("-> ternary op example val is {ternary_op_example}");

    for x in (1..10).rev() {
        print!("{x}");
    }
    println!();

    println!("Loop result is {loop_result}");
}

fn double_an_int(an_int: u32) -> u32 {
    //return an_int * 2;
    // or return last expression implicitly
    an_int * 2 // no semicolon ; is required here, it would turn this line into a statement, and function would return () unit, instead of u32
}
