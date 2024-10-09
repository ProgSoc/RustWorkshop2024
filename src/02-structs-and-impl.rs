// Structs can be defined in a variety of formats.

// Tuple struct: similar to regular structs, but the fields have no names.
// The fields are accessed just like tuples.
// struct Position(i32, i32);

// Regular struct: the fields are named.
// Optional `pub` for each field.
struct Position {
    x: i32,
    y: i32,
}

// Impl block. Method definitions are separated from data in Rust.
impl Position {

    // For methods like getters, you won't need to mutate the state of the object.
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    // For methods like setters, you will need to mutate the state.
    // Hence, we separate between `&self` and `&mut self`.
    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }
}

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let mut buffer = String::new();
    _ = std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

fn main() {

    // Rather than using the separate `x` and `y`, we have the one `pos` variable.
    let mut pos = Position { x: 0, y: 0 };

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        // We use the getter and setter methods that we implemented above.
        match &direction as &str {
            "left" => {
                println!("Moved left");
                pos.set_x(pos.get_x() - 1);
            }
            "right" => {
                println!("Moved right");
                pos.set_x(pos.get_x() + 1);
            }
            "up" => {
                println!("Moved up");
                pos.set_y(pos.get_y() + 1);
            }
            "down" => {
                println!("Moved down");
                pos.set_y(pos.get_y() - 1);
            }
            _ => {}
        }
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
}