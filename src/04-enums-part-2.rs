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

// Here, we have an enum that can contain data depending on the variant.
enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

// Demonstrating the power of other enums, and more importantly
// the many ways you can use implicit returns and match statements.
impl Action {
    fn from_string(action: &str) -> Option<Self> {
        match action {
            "splash" => Some(Self::Splash),
            "left" | "right" | "up" | "down" => {
                let direction = Direction::from_string(action).unwrap();
                Some(Self::Movement(direction))
            }
            a if action.starts_with("deposit") => {
                let sequence: Vec<&str> = action.split_whitespace().collect();
                if sequence.len() == 2 {
                    if let Ok(int_value) = sequence[1].parse::<u64>() {
                        Some(Self::Deposit(int_value))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
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
    let mut buffer = String::new();
    _ = std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

fn main() {

    let mut money = 0;
    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input action: ").to_lowercase();

        let maybe_action = Action::from_string(&direction_string as &str);

        if let Some(action) = maybe_action {
            match action {
                Action::Splash => {
                    println!("Splash");
                }
                Action::Movement(d) => {
                    println!("Moved.");
                    pos = pos.make_movement(d);
                }
                Action::Deposit(value) => {
                    println!("Deposited {value}.");
                    money += value;
                }
            }
        } else {
            println!("Invalid input.");
        }
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
    println!("Final money: ${money}");
}