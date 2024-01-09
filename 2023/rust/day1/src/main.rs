use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    
    let lines = read_input();

    let mut sum: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line.chars()
            .filter(|c| is_number(&c))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let number: u32 = numbers[0] * 10 + numbers[numbers.len() - 1];
        sum += number;
    }

    println!("The sum of the numbers are: {}", sum);
}

fn part2() {
    
    let lines = read_input();

    let mut sum: u32 = 0;

    for line in lines {
        let adjusted_line = line.replace("one", "o1e")
                                    .replace("two", "t2o")
                                    .replace("three", "th3ee")
                                    .replace("four", "f4ur")
                                    .replace("five", "f5ve")
                                    .replace("six", "s6x")
                                    .replace("seven", "se7en")
                                    .replace("eight", "ei8ht")
                                    .replace("nine", "ni9e");
        let numbers: Vec<u32> = adjusted_line.chars()
            .filter(|c| is_number(&c))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let number: u32 = numbers[0] * 10 + numbers[numbers.len() - 1];
        println!("{} {} {}", line, adjusted_line, number);
        sum += number;
    }

    println!("The sum of the numbers with the new rules are: {}", sum);

}

fn is_number(val: &char) -> bool {
    let digit = val.to_digit(10);
    return digit != None && digit >= Some(1) && digit <= Some(9);   
}

fn read_input() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
}
