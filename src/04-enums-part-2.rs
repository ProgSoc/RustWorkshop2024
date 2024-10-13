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

// Here, we have an enum that can contain data depending on the variant.
enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

// Demonstrating the power of other enums, and more importantly
// the many ways you can use implicit returns and match statements.
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

    // We remove `set_x` and `set_y` since we no longer use them,
    // and we are reminded to do this by the compiler.

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

    let mut money = 0;
    let mut pos = Position::new_position();

    println!("Start position: ({}, {})", pos.get_x(), pos.get_y());

    for _ in 0..6 {
        let direction_string = input("Input action: ").to_lowercase();

        let maybe_action = Action::from_string(&direction_string as &str);

        match maybe_action {
            Ok(action) => {
                match action {
                    Action::Splash => {
                        println!("Splash.");
                    }
                    Action::Movement(d) => {
                        match d {
                            Direction::Left => println!("Moved left."),
                            Direction::Right => println!("Moved right."),
                            Direction::Up => println!("Moved up."),
                            Direction::Down => println!("Moved down."),
                        }
                        pos = pos.make_movement(&d);
                    }
                    Action::Deposit(value) => {
                        println!("Deposited {value}.");
                        money += value;
                    }
                }
            }
            Err(error_msg) => {
                println!("{error_msg}");
            }
        }
    }

    println!("End position: ({}, {})", pos.get_x(), pos.get_y());
    println!("Final money: ${money}");
}