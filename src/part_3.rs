#![allow(unused)]

use std::io::Write;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Impl blocks work for enums as well.
// Let's make a static method called `from_string`
// that takes a string (`&str` in this case)
// and converts that string value into the corresponding direction.
// If an invalid direction is passed, we won't give back a valid value.

struct Position {
    x: i32,
    y: i32,
}

impl Position {

    fn new_position() -> Self {
        Self { x: 0, y : 0 }
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    // Just a quick note that we don't actually use these setter methods
    // at this point, and the compiler will warn us about this (how nice!)
    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }

    // While we're at it, let's add some more functionality to the
    // `Position` struct. Make an instance method `make_movement`
    // that takes a reference to a `Direction` object
    // and returns a new `Position` instance that represents a movement
    // one unit in the given direction.

    // For example, if we pass `&Direction::Left` to `Position { x: 3, y: 4 }`,
    // we want to be given a new object `Position { x: 2, y: 4 }`.
}

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()
}

pub fn main() {

    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input direction to move: ").to_lowercase();

        // Notice how we are able to represent directions
        // directly as enums now, so we don't need the `direction_index`
        // method anymore. Let's refactor this part of the code to reflect this.
        
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
}