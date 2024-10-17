// TODO: Remove this warning suppressor when you start working on Part 1.
#![allow(unused)]

use std::io::Write;

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
    // Do we need to fix anything?

    // TODO: Fix the variable declaration to allow mutation.
    let x = 0;
    let y = 0;

    println!("Start position: ({x}, {y})");

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        // Now let's actually use this value meaningfully.
        direction_index(&direction);

        // TODO: Implement some sort of branching using the above value.
    }

    // TODO: Use the `println!` macro here to print the end position.
}
