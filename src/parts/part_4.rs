// TODO: Remove this warning suppressor when you start working on Part 4.
#![allow(unused)]

use std::io::Write;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Impl blocks work for enums as well.
impl Direction {
    fn from_string(direction: &str) -> Option<Self> {
        match direction {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

// Here, we have an enum that can contain data depending on the variant.
enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

// Now let's make a `from_string` method for `Action`
// that acts just like `Direction::from_string`, but instead of `Option<Direction>`
// we will output a `Result<Action, String>`,
// and if we fail to create a valid action, we will put in a suitable error message.

// TODO: Use an `impl` block to implement `from_string` for the `Action` enum.

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new_position() -> Self {
        Self { x: 0, y: 0 }
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }

    fn make_movement(&self, dir: &Direction) -> Self {
        match dir {
            &Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            &Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            &Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            &Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()
}

pub fn main() {

    // We gotta keep track of money too.

    // TODO: Add a `money` variable here of type `i32`, which is mutable.

    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input action: ").to_lowercase();

        // We're going to refactor this part again.
        // We have to more layers this time around,
        // since there's a `Result<Action, String>`
        // and the Action may contain a `Direction`.

        // TODO: Use a `match` statement here to code the main running logic.
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());

    // Let's also print the money we have at the end.

    // TODO: Use `println!` to print the money at the end.
}
