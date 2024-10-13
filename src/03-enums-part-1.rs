use std::io::Write;

enum Direction {
    Left, Right, Up, Down
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

    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }

    // Let's add some more functionality to the `Position` struct as well,
    // and let it interact with the `Direction` enum.
    fn make_movement(&self, dir: &Direction) -> Self {
        match dir {
            &Direction::Left => Position { x: self.x - 1, y: self.y },
            &Direction::Right => Position { x: self.x + 1, y: self.y },
            &Direction::Up => Position { x: self.x, y: self.y + 1 },
            &Direction::Down => Position { x: self.x, y: self.y - 1 },
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()
}

fn main() {

    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input direction to move: ").to_lowercase();

        let maybe_direction = Direction::from_string(&direction_string as &str);

        if let Some(direction) = maybe_direction {
            pos = pos.make_movement(&direction);
            match direction {
                Direction::Left => println!("Moved left."),
                Direction::Right => println!("Moved right."),
                Direction::Up => println!("Moved up."),
                Direction::Down => println!("Moved down."),
            }
        } else {
            println!("Invalid input.");
        }
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
}