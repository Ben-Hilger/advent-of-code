use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

struct Instructions {
    operation: String,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize
}

fn part1() {

    let lines = read_file();

    let size: usize = 1000;

    let mut number_of_lighs_on = 0;
    let mut arr: Vec<bool> = vec![false; size * size];

    for line in lines {
        let instructions = process_instructions(&line);

        for current_x in instructions.start_x..=instructions.end_x {
            for current_y in instructions.start_y..=instructions.end_y {
                let position = current_y * size + current_x;
                if instructions.operation == "toggle" {
                    if arr[position] {
                        number_of_lighs_on -= 1;
                    } else {
                        number_of_lighs_on += 1;
                    }

                    arr[position] = !arr[position];
                } else if instructions.operation == "on" {
                    if !arr[position] {
                        number_of_lighs_on += 1;
                    }
                    arr[position] = true
                } else if instructions.operation == "off" {
                    if arr[position] {
                        number_of_lighs_on -= 1;
                    }
                    arr[position] = false;
                }
            }
        }
    }

    println!("There are {} lights on", number_of_lighs_on);
}

fn part2() {

    let lines = read_file();

    let size: usize = 1000;

    let mut total_brightness = 0;
    let mut arr: Vec<i32> = vec![0; size * size];

    for line in lines {
        let instructions = process_instructions(&line);

        for current_x in instructions.start_x..=instructions.end_x {
            for current_y in instructions.start_y..=instructions.end_y {
                let position = current_y * size + current_x;
                if instructions.operation == "toggle" {
                    total_brightness += 2;
                    arr[position] += 2;
                } else if instructions.operation == "on" {
                    total_brightness += 1;
                    arr[position] += 1;
                } else if instructions.operation == "off" {
                    if arr[position] == 0 {
                        continue;
                    }
                    arr[position] -= 1;
                    total_brightness -= 1;
                }
            }
        }
    }

    println!("The total brightness is: {}", total_brightness);
}


fn process_instructions(line: &String) -> Instructions {
   let current_line = line.replace("turn ", "");

   let parts = current_line.split(" ").collect::<Vec<_>>();
   
   let operation = parts[0];

   let start_pos = parts[1].split(",").collect::<Vec<_>>();
   let end_pos = parts[3].split(",").collect::<Vec<_>>();

   return Instructions { 
                        operation: operation.to_string(), 
                        start_x: start_pos[0].parse().unwrap(),
                        start_y: start_pos[1].parse().unwrap(),
                        end_x: end_pos[0].parse().unwrap(),
                        end_y: end_pos[1].parse().unwrap()
                    };
}

fn read_file() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

