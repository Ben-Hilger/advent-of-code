use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = read_input();

    let size = 150;

    let mut x = size / 2;
    let mut y = size / 2;

    let mut number_of_homes_visited = 1;
    let mut arr: Vec<bool> = vec![false; size * size];
    arr[y * size + x] =  true;
    
    for instruction in contents.chars() {
        if instruction == '^' {
            y  -= 1;
        } else if instruction == 'v' {
            y += 1;
        } else if instruction == '>' {
            x += 1;
        } else if instruction == '<' {
            x -= 1;
        }

        if !arr[y * size + x] {
            arr[y * size + x] = true;
            number_of_homes_visited += 1;
        }
    }

    println!("At least {} homes got presents", number_of_homes_visited);
}

fn part2() {
    let contents = read_input();

    let size = 150;

    let mut arr: Vec<bool> = setup_array(size);

    let mut santa_x = size / 2;
    let mut santa_y = size / 2;

    let mut robot_santa_x = size / 2;
    let mut robot_santa_y = size / 2;

    let mut number_of_homes_visited = 1;

    arr[santa_y * size + santa_x] = true;

    for (index, instruction) in contents.chars().enumerate() {
        let mut x_pointer = &mut santa_x;
        let mut y_pointer = &mut santa_y;
        // Even = robot santa moves
        if index % 2 == 0 {
            x_pointer = &mut robot_santa_x;
            y_pointer = &mut robot_santa_y;
        }

        if instruction == '^' {
            *y_pointer  -= 1;
        } else if instruction == 'v' {
            *y_pointer += 1;
        } else if instruction == '>' {
            *x_pointer += 1;
        } else if instruction == '<' {
            *x_pointer -= 1;
        }
        
        if !arr[*y_pointer * size + *x_pointer] {
            arr[*y_pointer * size + *x_pointer] = true;
            number_of_homes_visited += 1;
        }

    }

    println!("At least {} homes got presents", number_of_homes_visited);
}

fn setup_array(size: usize) -> Vec<bool> {
    return vec![false; size * size];
}

fn read_input() -> String {
    return read_to_string("input.txt")
        .expect("Should be able to read the input file");
}
