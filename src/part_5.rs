#![allow(unused)]

// Import traits from the standard library (std),
// which we can implement for types that we define in our program.
// We'll need to add more to this statement.
use std::io::Write;

// Is there a way to make `Direction` easier to work with?
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

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

// Let's implement the `fmt::Display` trait for `Direction`
// so that we can put these objects directly into a `println!` macro.

// Is there a way to make `Action` easier to work with?
enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

impl Action {
    fn from_string(action: &str) -> Result<Self, String> {
        match action {
            "splash" => Ok(Self::Splash),
            "left" | "right" | "up" | "down" => {
                let direction = Direction::from_string(action).unwrap();
                Ok(Self::Movement(direction))
            }
            _ if action.starts_with("deposit") => {
                let sequence: Vec<&str> = action.split_whitespace().collect();
                if sequence.len() == 2 {
                    if let Ok(int_value) = sequence[1].parse::<u64>() {
                        Ok(Self::Deposit(int_value))
                    } else {
                        Err("Invalid integer.".to_string())
                    }
                } else {
                    Err("'deposit' must be followed by a single integer.".to_string())
                }
            }
            _ => Err("Invalid action.".to_string()),
        }
    }
}

// Let's also implement `fmt::Display` for `Action`,
// and we can rely on the definition for `fmt::Display` for `Direction` as well.

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new_position() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new_position()
    }
    // When we just print it out via the `Display` trait,
    // we won't need to implement getters for `Position` in this program.
    // We just remove them to make the compiler a little happier.
}

// Implement the `fmt::Display` trait for `Position`.

// Let's also make it possible to do something like
// `Position` + `Direction`. What trait would we implement
// to enable this functionality?

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()
}

pub fn main() {

    let mut money = 0;
    
    let mut pos = Position::new_position();

    // Relies on the `Display` trait being implemented for `Position`.
    // println!("Start position: {}", pos);

    for _ in 0..6 {
        let direction_string = input("Input action: ").to_lowercase();

        match Action::from_string(&direction_string) {
            Ok(action) => {
                // We can put the printing functionality just in one line
                // outside the `match` statement.
                // Though of course, this relies on the `fmt::Display` trait
                // being implemented for `Action`.
                // println!("You {action}.");

                match action {
                    Action::Splash => {}
                    Action::Movement(d) => {
                        // This will need some trait to overload the operator.
                        // pos = pos + d;
                    }
                    Action::Deposit(value) => {
                        money += value;
                    }
                }
            }
            Err(error_msg) => {
                println!("{error_msg}");
            }
        }
    }

    // Implementing the `Display` trait also gives you
    // access to the `to_string` method.
    // println!("End position: {}", pos.to_string());
    println!("Final money: ${money}");
}
