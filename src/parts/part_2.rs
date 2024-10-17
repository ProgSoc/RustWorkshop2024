// TODO: Remove this warning suppressor when you start working on Part 2.
#![allow(unused)]

use std::io::Write;

// Let's make a `Position` struct that represents a 2D point.
// It will have the following methods:
// * A static method `new_position`, that returns the origin `Position`.
// * Instance methods `get_x` and `get_y`, which return the current x and y values.
// * Instance methods `set_X` and `set_y`, which let you modify the x and y values.

// TODO: Implement the `Position` struct: both the struct itself and its methods.

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()
}

fn direction_index(dir: &str) -> i32 {
    match dir {
        "left" => 0,
        "right" => 1,
        "up" => 2,
        "down" => 3,
        _ => 4,
    }
}

pub fn main() {
    // Instead of using `x` and `y`, we can make a `Position` struct.

    // TODO: Declare a mutable variable that is of type `Position`.


    // TODO: Uncomment the line below and fill it in with the correct values.
    // println!("Start position: ({}, {})", /* FILL IN */);

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        match direction_index(&direction) {
            0 => {
                println!("Moved left.");
                // TODO: Implement the equivalent of `x -= 1;`.
            }
            1 => {
                println!("Moved right.");
                // TODO: Implement the equivalent of `x += 1;`.
            }
            2 => {
                println!("Moved up.");
                // TODO: Implement the equivalent of `y += 1;`.
            }
            3 => {
                println!("Moved down.");
                // TODO: Implement the equivalent of `y -= 1;`.
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }

    // TODO: Use the `println!` macro here to print the end position.
}
