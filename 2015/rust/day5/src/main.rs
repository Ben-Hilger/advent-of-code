use std::fs::read_to_string;
use std::cmp::min;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_file();

    let mut number_nice_str = 0;

    for line in lines {

        if contains_forbidden_strings(&line) {
            continue;
        }
        
        let mut number_of_vowels = 0;
        let mut has_same_character_in_a_row = false;
        let mut previous_char = ' ';

        for char in line.chars() {
            if char == previous_char && !has_same_character_in_a_row {
                has_same_character_in_a_row = true;
            }
            
            previous_char = char;

            if is_vowel(char) {
                number_of_vowels += 1;
            }
        }

        if number_of_vowels >= 3 && has_same_character_in_a_row {
            number_nice_str += 1;
        }
    }

    println!("There are {} nice string(s)", number_nice_str);
}

fn is_vowel(character: char) -> bool {
    return "aeiou".contains(character);
}

fn contains_forbidden_strings(line: &String) -> bool {
    let forbidden_strings = [
        "ab",
        "cd",
        "pq",
        "xy"
    ];
    for forbidden_string in forbidden_strings {
        if line.contains(forbidden_string) {
            return true;
        }
    }
    return false;
}

fn part2() {
    let lines = read_file();

    let mut number_nice_str = 0;

    for line in lines {
        if has_pair_of_pairs(&line) && has_pair_that_repeats_after_break(&line) {
            number_nice_str += 1;
        }
    }

    println!("There are {} nice string(s) with the new rules", number_nice_str);

}

fn has_pair_that_repeats_after_break(line: &String) -> bool {

    let mut previous_previous_char = ' ';
    let mut previous_char = ' ';

    for char in line.chars() {
        if previous_previous_char == char {
            return true;
        } else {
            previous_previous_char = previous_char;
                previous_char = char;
        }
    }

    return false;
}

struct Pair {
    pair: String,
    pos: usize
}

fn has_pair_of_pairs(line: &String) -> bool {
    let mut arr: Vec<Pair> = Vec::with_capacity(line.len());
    
    for (index, _) in line.chars().enumerate() {
        let slice: String = line[index..min(line.len(), index+2)].to_string();
        let pair = Pair { pair: slice.to_string(), pos: index };
        let not_found = arr.iter()
            .filter(|p| p.pair == slice)
            .filter(|p| p.pos != index-1)
            .collect::<Vec<_>>()
            .is_empty();
        if !not_found {
            return true;
        }
        arr.push(pair);
    }

    return false;
}

fn read_file() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

