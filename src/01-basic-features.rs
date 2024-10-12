use std::io::Write;

// Here we have a function, and notice how it takes a reference to a string.
// It will read the input value, but never really consume or move it.

// Notice also that the return type is the owned `String` type,
// since we're creating a new value within the function and returning it.
fn input(prompt: &str) -> String {

    // We use the `prompt` value here.
    print!("{prompt}");

    // We're going to have to hand-wave this part away,
    // but this just flushes the buffer so that the prompt prints,
    // and then reads a single line from the user input.
    let _ = std::io::stdout().flush();
    std::io::stdin().lines().next().unwrap().unwrap()

    // Also note that we returned using an implicit return.
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

fn main() {

    // The x-coordinate and y-coordinate will change within the program.
    let mut x = 0;
    let mut y = 0;

    // Show where we are to start.
    println!("Start position: ({x}, {y})");

    for _ in 0..6 {

        let direction = input("Input direction to move: ").to_lowercase();

        // We show that `match` statements will match values (not just structure).
        match direction_index(&direction) {
            0 => {
                println!("Moved left");
                x -= 1;
            }
            1 => {
                println!("Moved right");
                x += 1;
            }
            2 => {
                println!("Moved up");
                y += 1;
            }
            3 => {
                println!("Moved down");
                y -= 1;
            }
            // Remember, `match` statements have to cover all possibilities.
            _ => {
                println!("Invalid input.");
            }
        }
    }

    println!("End position: ({x}, {y})");
}