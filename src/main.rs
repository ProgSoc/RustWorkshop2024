mod parts;
mod solutions;

fn main() {

    // Toggle between each of the parts of the workshop,
    // and also maybe trying out the pre-written solution code.
    let use_solution = false;
    let part = 1;

    if use_solution {
        match part {
            1 => solutions::part_1::main(),
            2 => solutions::part_2::main(),
            3 => solutions::part_3::main(),
            4 => solutions::part_4::main(),
            5 => solutions::part_5::main(),
            _ => {}
        }
    } else {
        match part {
            1 => parts::part_1::main(),
            2 => parts::part_2::main(),
            3 => parts::part_3::main(),
            4 => parts::part_4::main(),
            5 => parts::part_5::main(),
            _ => {}
        }
    }
}
