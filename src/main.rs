mod part_1;
mod part_2;
mod part_3;
mod part_4;
mod part_5;

mod solutions;

fn main() {

    // Toggle between each of the parts of the workshop,
    // and also maybe trying out the pre-written solution code.
    let solution = false;
    let part = 1;

    if solution {
        match part {
            1 => part_1::main(),
            2 => part_2::main(),
            3 => part_3::main(),
            4 => part_4::main(),
            5 => part_5::main(),
            _ => {}
        }
    } else {
        match part {
            1 => solutions::part_1::main(),
            2 => solutions::part_2::main(),
            3 => solutions::part_3::main(),
            4 => solutions::part_4::main(),
            5 => solutions::part_5::main(),
            _ => {}
        }
    }
}
