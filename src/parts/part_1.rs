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
    let x = 0;
    let y = 0;

    println!("Start position: ({x}, {y})");

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        // Now let's actually use this value meaningfully.
        direction_index(&direction);
    }

    // Now let's output the end position as well.
}
