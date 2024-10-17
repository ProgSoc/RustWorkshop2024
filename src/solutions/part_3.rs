use std::io::Write;

// Here we have an enum without any fields in the variants.
// This functions somewhat equivalently to enums in other languages,
// except do beware that there isn't an automatically implemented
// direct mapping to integers in this case.
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Impl blocks are allowed for enums as well, not just structs.
impl Direction {

    // Now, while you can consider this static method as a constructor for
    // a `Direction` object, there are cases where we would be given a string
    // that doesn't correspond to a valid direction.
    // This is where the standard library's `Option<T>` enum comes in.
    // Rather than having null values which must be explicitly accounted for
    // in many cases, we have a return type that specifically indicates that
    // a valid value might not be returned.
    // Valid values will contain a `Direction` variant inside them,
    // meanwhile invalid values are just an `Option::None` instance.
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
    #[allow(dead_code)]
    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    #[allow(dead_code)]
    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }

    // Notice how here, we're only taking a borrow to the `Direction` object.
    // Although the enum this time just represents one of multiple values,
    // by default, when you move an enum like this, it will not be copyable.
    // We will learn how to change this a bit later.
    fn make_movement(&self, dir: &Direction) -> Self {
        // Just remember, everything here is also an implicit return.
        // Each arm of the match statement is an expression of its own
        // (in this particular case, a `Position` instance).
        match dir {
            &Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            &Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            &Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            &Direction::Down => Self {
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

    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input direction to move: ").to_lowercase();

        // We don't need `direction_index` anymore, since that job is
        // done by the `Direction::from_string` method.
        // We still use a match though.
        match Direction::from_string(&direction_string) {
            Some(d) => {
                match d {
                    Direction::Left => println!("Moved left."),
                    Direction::Right => println!("Moved right."),
                    Direction::Up => println!("Moved up."),
                    Direction::Down => println!("Moved down."),
                }
                pos = pos.make_movement(&d);
            }
            None => {
                println!("Invalid input.");
            }
        }
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
}