use std::io::Write;

// Here's our `Position` struct that represents a 2D point in a grid,
// each coordinate being a signed 32-bit integer.

// Note that we could have declared the fields using `pub x: i32`,
// which would make the fields accessible from any point in the program,
// and modifiable if the object is marked as mutable.
// We don't use `pub` in this manner here, just so we have an excuse
// to implement getters/setters.
struct Position {
    x: i32,
    y: i32,
}

impl Position {

    // Static methods are differentiated from instance methods
    // by the lack of a `self`, `&self` or `&mut self` argument.
    // Regardless, you will still have access to the `Self` type,
    // which is just whichever type you're implementing functions for.
    fn new_position() -> Self {

        // You can always construct structs from its fields
        // using this notation, even within static methods.
        // You can use static methods to define
        // your own constructors if you wish,
        // but this option is always available.
        Self { x: 0, y: 0 }
    }

    // Implementing getters and setters in Rust also shows
    // an important difference between two types of borrowing.
    // First of all, you don't want to consume
    // an object when you read/write from/to it,
    // so of course we don't use `self` as-is.
    // A getter only needs to read a value,
    // and thus we only take an immutable borrow.
    // Meanwhile, setters need to modify the object's value,
    // so we need a mutable borrow.

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
}

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
    // Remember that we will be mutating it just as we did for `x` and `y`,
    // so we use `let mut` as previously.
    // Our static method `new_position` acts like a constructor of sorts.
    let mut pos = Position::new_position();

    // The formatting abilities of the `println!` macro are
    // a bit limited: you can put variable names inside the string literal
    // enclosed with curly braces, but if the expression is more complicated
    // you'll have to pass them as separate arguments to the macro instead.
    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        match direction_index(&direction) {
            0 => {
                println!("Moved left.");
                // The equivalent of `x -= 1`,
                // using our getter and setter for `Position.x`.
                pos.set_x(pos.get_x() - 1);
            }
            1 => {
                println!("Moved right.");
                // The equivalent of `x += 1`,
                // using our getter and setter for `Position.x`.
                pos.set_x(pos.get_x() + 1);
            }
            2 => {
                println!("Moved up.");
                // The equivalent of `y += 1`,
                // using our getter and setter for `Position.y`.
                pos.set_y(pos.get_y() + 1);
            }
            3 => {
                println!("Moved down.");
                // The equivalent of `y -= 1`,
                // using our getter and setter for `Position.y`.
                pos.set_y(pos.get_y() - 1);
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }

    // Don't forget our end position here.
    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
}
