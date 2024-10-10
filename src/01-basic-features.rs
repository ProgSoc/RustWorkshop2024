// Here we have a function, and notice how it takes a reference to a string.
// It will read the input value, but never really consume or move it.

// Notice also that the return type is the owned `String` type,
// since we're creating a new value within the function and returning it.
fn input(prompt: &str) -> String {
    // We use the `prompt` value here.
    print!("{prompt}");

    // The way reading from CLI works in Rust is by first preparing a String variable
    // to act as the buffer for the inputted string.
    let mut buffer = String::new();

    // `std` is the Rust standard library.
    // The `read_line` function reads from CLI and puts the string into the given buffer.
    _ = std::io::stdin().read_line(&mut buffer);

    // We do the funky little "implicit return" here.
    buffer.trim().to_string()
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
        match &direction as &str {
            "left" => {
                println!("Moved left");
                x -= 1;
            }
            "right" => {
                println!("Moved right");
                x += 1;
            }
            "up" => {
                println!("Moved up");
                y += 1;
            }
            "down" => {
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