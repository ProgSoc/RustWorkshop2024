// Demonstrate `cargo add`.
use input_macro::input;

// Demonstrate other imports from std.
use std::time::Instant;

// Typedef.
type Position = (usize, usize);

// Demonstrating Rust enums, with all three styles of variants.
#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Trap(i32, &'static str),
    Goal(i32),
    Teleport { x: usize, y: usize },
}

// Another demonstration of a Rust enum.
#[derive(Clone, Copy, Debug)]
enum Direction {
    Left, Right, Up, Down
}

// Demonstrating impl blocks, as well as `match` statements as expressions.
// Also demonstrating another common enum: Option.
impl Direction {
    // Currently only demonstrates static methods, not member functions yet.
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

// Demonstrating trait implementation.
impl std::ops::Add<Direction> for Position {
    type Output = Self;
    fn add(self, d: Direction) -> Position {
        match d {
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Up => (self.0, self.1 + 1),
            Direction::Down => (self.0, self.1 - 1),
        }
    }
}

// Demonstrating structs.
#[derive(Debug)]
struct Choice {
    pub movement: Direction,
    pub time_taken: u128,
}

// Demonstrating generics, more specific const generics (not necessarily just types).
// Also demonstrating another common enum: Result.
fn next_position<const X: usize, const Y: usize>(
    pos: Position,
    d: Direction,
) -> Result<Position, &'static str> {
    // Showing matches again, but with guards this time.
    match d {
        Direction::Left if pos.0 == 0 => Err("Cannot move further left"),
        Direction::Right if pos.0 == X - 1 => Err("Cannot move further right"),
        Direction::Up if pos.1 == Y - 1 => Err("Cannot move further up"),
        Direction::Down if pos.1 == 0 => Err("Cannot move further down"),
        _ => Ok(pos + d),
    }

    /*
    // Alternative implementation, before operator overloading happens
    match d {
        Direction::Left => {
            if pos.0 == 0 {
                Err("Cannot move further left")
            } else {
                Ok((pos.0 - 1, pos.1))
            }
        }
        Direction::Right => {
            if pos.0 == X - 1 {
                Err("Cannot move further right")
            } else {
                Ok((pos.0 + 1, pos.1))
            }
        }
        Direction::Up => {
            if pos.1 == Y - 1 {
                Err("Cannot move further up")
            } else {
                Ok((pos.0, pos.1 + 1))
            }
        }
        Direction::Down => {
            if pos.1 == 0 {
                Err("Cannot move further down")
            } else {
                Ok((pos.0, pos.1 - 1))
            }
        }
    }
    */
}

fn main() {

    // Compile-time constants: (equivalent to constexpr in >=C++11)
    const GRID_X_LIMIT: usize = 4;
    const GRID_Y_LIMIT: usize = 4;

    // Stack-allocated array. Note the compile-time constant array size.
    let map: [[Tile; GRID_X_LIMIT]; GRID_Y_LIMIT] = [
        [Tile::Empty, Tile::Empty, Tile::Trap(1, "Wrong way!"), Tile::Empty],
        [Tile::Empty, Tile::Wall, Tile::Wall, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Teleport { x: 3, y: 0 }, Tile::Teleport { x: 0, y: 1 }],
        [Tile::Trap(2, "Too far!"), Tile::Empty, Tile::Empty, Tile::Goal(3)],
    ];

    // Mutable variables. Mutation is opt-in, and is more or less independent of variable type.
    let mut position: Position = (0, 0);

    let mut points: i32 = 0;

    let mut game_history: Vec<Choice> = Vec::new();

    // An infinite loop in Rust. Usable with `continue` and `break` statements.
    loop {

        let start_time = Instant::now();
        let string_direction = input!("Which direction would you like to move? ").to_lowercase();
        let choice_time = start_time.elapsed().as_millis();

        // Static method calling.
        let direction = Direction::from_string(&string_direction);

        // Useful utility function with Option<T>.
        // Note that structural matching will cause a *move*.
        if direction.is_none() {
            println!("Please input a direction that is one of: 'left', 'right', 'up', 'down'.");
            continue;
        }

        // Struct initialisation.
        game_history.push(Choice { movement: direction.unwrap(), time_taken: choice_time });

        // Calling with generic parameters.
        match next_position::<GRID_X_LIMIT, GRID_Y_LIMIT>(position, direction.unwrap()) {
            Ok((new_x, new_y)) => {
                match map[new_y][new_x] {
                    Tile::Empty => {
                        println!("Moved successfully to ({new_x}, {new_y}).");
                        position = (new_x, new_y);
                    }
                    Tile::Wall => {
                        println!("Cannot walk into a wall at ({new_x}, {new_y}).");
                        println!("Remaining at ({0}, {1}).", position.0, position.1);
                    }
                    Tile::Trap(penalty, taunt_message) => {
                        println!("Trap!! {taunt_message}");
                        println!("Deducted {penalty} points.");
                        position = (new_x, new_y);
                        points -= penalty;
                    }
                    Tile::Goal(reward) => {
                        println!("You reached the goal!");
                        println!("Gained {reward} points.");
                        points += reward;
                        break;
                    }
                    Tile::Teleport { x, y } => {
                        println!("Teleported from ({new_x}, {new_y}) to ({x}, {y}).");
                        position = (x, y);
                    }
                }
            }
            Err(error_message) => {
                println!("{error_message}.");
            }
        }
    }

    println!("You have {points} points.");
    println!("Your game history was:");
    for choice in &game_history {
        println!("Took {1} ms to move {0:?}.", choice.movement, choice.time_taken);
    }
    game_history.push(Choice { movement: Direction::Left, time_taken: 0 });
}
