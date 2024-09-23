fn input(prompt: &str) -> String {
    print!("{prompt}");
    let mut buffer = String::new();
    _ = std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

fn main() {

    let mut x = 0;
    let mut y = 0;

    for _ in 0..6 {
        let direction = input("Input direction to move: ").to_lowercase();
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
            _ => {}
        }
    }

    println!("{x}, {y}");
}