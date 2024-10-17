#![allow(unused)]

use std::io::Write;

// Let's make a `Position` struct that represents a 2D point.
// It will have the following methods:
// * A static method `new_position`, that returns the origin `Position`.
// * Instance methods `get_x` and `get_y`, which return the current x and y values.
// * Instance methods `set_X` and `set_y`, which let you modify the x and y values.

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

    println!("Start position: ", /* What do I put here? */);

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        match direction_index(&direction) {
            0 => {
                println!("Moved left.");
                // How do we do the equivalent of `x -= 1;`?
            }
            1 => {
                println!("Moved right.");
                // How do we do the equivalent of `x += 1;`?
            }
            2 => {
                println!("Moved up.");
                // How do we do the equivalent of `y += 1;`?
            }
            3 => {
                println!("Moved down.");
                // How do we do the equivalent of `y -= 1;`?
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }

    // Now let's output the end position as well.
}
