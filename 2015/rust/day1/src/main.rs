use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = read_input();
   
    let mut current_floor = 0;

    for instruction in contents.chars() {
        if instruction == '(' {
            current_floor += 1;
        } else if instruction == ')' {
            current_floor -= 1;
        }
    }

    println!("The floor santa needs to go to is: {current_floor}");
}

fn part2() {
    let contents = read_input();

    let mut current_floor = 0;

    for (i, instruction) in contents.chars().enumerate() {
        if instruction == '(' {
            current_floor += 1;
        } else if instruction == ')' {
            current_floor -= 1;
        }
        if current_floor == -1 {
            let position = i + 1;
            println!("Santa enters the basement at position {position}");
            break;
        }
    }

}

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("Should be able to read the input!");
    return contents;
}
