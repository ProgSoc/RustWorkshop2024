use std::io::Write;

// We're kind of hand-waving this part away for the entire workshop,
// but the jist of it is this:
// We print out a prompt without a newline character,
// and then flush the standard output so that the prompt appears on-screen.
// Then we read one line from the console, assuming absolutely nothing goes wrong.
// Having `.unwrap().unwrap()` just lying there doesn't sit well at all,
// but this is really all for the sake of convenience.
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

    // Since we'll be mutating our position,
    // which is expressed in terms of `x` and `y`,
    // we need to mark these variables as mutable.
    let mut x = 0;
    let mut y = 0;

    // The `println!` macro takes a format string,
    // and automatically binds the values of variables
    // when the variable names are found inside curly brackets.
    println!("Start position: ({x}, {y})");

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();

        // We're hard coding integers for now,
        // since we know exactly how `direction_index` works.
        // As you'll see later in the workshop,
        // there are better ways to go about this,
        // but to demonstrate `match` statements for now,
        // this is still useful.
        match direction_index(&direction) {
            0 => {
                println!("Moved left.");
                x -= 1;
            }
            1 => {
                println!("Moved right.");
                x += 1;
            }
            2 => {
                println!("Moved up.");
                y += 1;
            }
            3 => {
                println!("Moved down.");
                y -= 1;
            }
            // Of course, we need to make sure we cover
            // every possibility in a match statement.
            _ => {
                println!("Invalid input.");
            }
        }
    }

    println!("End position: ({x}, {y})");
}
