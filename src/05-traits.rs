// Import traits from the standard library (std),
// which we can implement for types that we define in our program.
use std::{
    default::Default,
    fmt,
    ops::Add,
};

enum Direction {
    Left, Right, Up, Down
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

// When implmenting the Display trait for enums,
// it also suffices to have a match statement that prints differently.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Left => write!(f, "left"),
            &Self::Right => write!(f, "right"),
            &Self::Up => write!(f, "up"),
            &Self::Down => write!(f, "down"),
        }
    }
}

enum Action {
    Splash,
    Movement(Direction),
    Deposit(u64),
}

impl Action {
    fn from_string(action: &str) -> Option<Self> {
        match action {
            "splash" => Some(Self::Splash),
            "left" | "right" | "up" | "down" => {
                let direction = Direction::from_string(action).unwrap();
                Some(Self::Movement(direction))
            }
            _ if action.starts_with("deposit") => {
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

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Splash => write!(f, "splashed"),
            // Relies on `Display` being implemented for `Direction`.
            &Self::Movement(ref d) => write!(f, "moved {d}"),
            &Self::Deposit(ref m) => write!(f, "deposited {m} credits"),
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
}

impl Default for Position {
    fn default() -> Self {
        Self::new_position()
    }
    // We remove `get_x` and `get_y` since we no longer use them,
    // and we are reminded to do this by the compiler.
}

// This is a very typical implemention of the `Display` trait.
// We can access the fields directly since this is a member function,
// so we don't need to use `get_x` and `get_y` here.
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Direction> for Position {
    type Output = Self;
    fn add(self, d: Direction) -> Position {
        match d {
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Up => Position { x: self.x, y: self.y + 1 },
            Direction::Down => Position { x: self.x, y: self.y - 1 },
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

    // Relies on the `Display` trait being implemented for `Position`.
    println!("Start position: {}", pos);

    for _ in 0..6 {
        let direction_string = input("Input action: ").to_lowercase();

        let maybe_action = Action::from_string(&direction_string as &str);

        if let Some(action) = maybe_action {
            // Uses the `Display` trait being implemented for `Action`.
            println!("You {action}.");
            match action {
                Action::Splash => {}
                Action::Movement(d) => {
                    pos = pos + d;
                }
                Action::Deposit(value) => {
                    money += value;
                }
            }
        } else {
            println!("Invalid input.");
        }
    }

    // Implementing the `Display` trait also gives you access to the `to_string` method.
    println!("End position: {}", pos.to_string());
    println!("Final money: ${money}");
}