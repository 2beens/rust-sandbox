use rand::{Rng, RngCore};
use std::cmp::Ordering;
use rust_sandbox::{
    module_one::ex_references::{change_str, get_str_len},
    module_structs::{
        user::{get_example_user, User},
    }
};

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

    // structs examples function
    structs_examples();

    print_something();

    let p1 = Person {
        name: String::from("Serj"),
        age: 35,
    };
    let name_len = get_str_len(&p1.name);
    println!("-> created person: {} of len {name_len}, age: {}", p1.name, p1.age);

    let mut my_string = String::from("hello Serj");
    change_str(&mut my_string);
    println!("my_string val after changed via fn: ${my_string}");
    let my_str_2 = "str literal";
    println!("my_str_2 len is: {}", get_str_len(my_str_2));

    loop {
        rand_num = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess) {
            Ok(input_size) => {
                guess = String::from(guess.trim());
                if guess == "quit" || guess == "q" {
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

    let a = give_a_slice();
    for (i, x) in a.iter().enumerate() {
        println!("{}th member of array is: {x}", i+1);
    }
}

fn double_an_int(an_int: u32) -> u32 {
    //return an_int * 2;
    // or return last expression implicitly
    an_int * 2 // no semicolon ; is required here, it would turn this line into a statement, and function would return () unit, instead of u32
}

fn give_a_slice() -> [i32; 5] {
    let a = [1, 2, 3, 4, 5];

    // let slice = a[1..3];
    // (a, slice)

    a
}

fn structs_examples() {
    let user1 = get_example_user();
    println!("> example user 1 email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("> example user 2 qfemail: {}", user2.email);
    println!("> example user 1 email again: {}", user1.email); // TODO: this should not work?

    // tuple structs
    struct Color(i32, i32, i32, String);
    struct Point(i32, i32, i32, String);
    // black and origin values are different types, because they’re instances of different tuple structs
    let black = Color(10, 0, 1, String::from("color"));
    let origin = Point(20, 0, 2, String::from("point"));

    println!("{} {}", black.0, black.3);
    println!("{} {}", origin.0, origin.3);
}
