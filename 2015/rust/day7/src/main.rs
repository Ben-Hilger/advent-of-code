use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    part1();
}

struct Wire {
    value: i32,
    instructions: Option<WireInstructions>
}

struct WireInstructions {
    wire1: String,
    instruction: String,
    wire2: Option<String>
}

fn part1() {

    let mut wires = HashMap::<String, Wire>::new();
    let lines = read_file();

    for line in lines {
        let wire_operations = line
                .split("->")
                .collect::<Vec<_>>();
   
        let wire_label = wire_operations.get(1).unwrap().trim().to_string();
        let insturctions = wire_operations.get(0).unwrap().trim().split(" ").collect::<Vec<_>>();

        let mut value = -1;
        let mut wire_instructions: Option<WireInstructions> = None;

        if insturctions.len() == 1 {
            let is_number = insturctions.get(0).unwrap().parse::<i32>().is_ok();
            if is_number {
                value = insturctions.get(0).unwrap().parse().expect("instructions with length 1 should be a number");        
            } else {
                let wire1 = insturctions.get(0).unwrap().to_string();
                wire_instructions = Some(WireInstructions { wire1, instruction: "DIRECT".to_string(), wire2: None});
            }
        } else if insturctions.len() == 2 && insturctions.get(0).unwrap().to_string() == "NOT".to_string() {
            let wire1 = insturctions.get(1).unwrap().to_string();
            wire_instructions = Some(WireInstructions { wire1 , instruction: "NOT".to_string(), wire2: None });  
        } else {
            let wire1 = insturctions.get(0).unwrap().to_string();
            let instruction = insturctions.get(1).unwrap().to_string();
            let wire2 = insturctions.get(2).unwrap().to_string();
            wire_instructions = Some(WireInstructions { wire1, instruction, wire2: Some(wire2) });
        }

        wires.insert(wire_label, Wire { value, instructions: wire_instructions });
    }

    let wire_to_evaluate = "a".to_string();

    let result = evaluate_wire(&wire_to_evaluate, &wires);
   
    println!("{}", result);
}

fn evaluate_wire(wire: &String, hash_map: &HashMap<String, Wire>) -> i32 {

    if let Some(wire_mapping) = hash_map.get(wire) {
        if wire_mapping.value >= 0 {
            return wire_mapping.value;
        }

        if let Some(instructions) = &wire_mapping.instructions {
            let wire1 = &instructions.wire1;
            let instruction = &instructions.instruction;

            let wire1_value: i32;
            if is_number(&wire1) {
                wire1_value = wire1.parse().expect("Should be a number!");
            } else {
                wire1_value = evaluate_wire(&wire1, hash_map);
            }
            if instruction == "NOT" {
                return !wire1_value;
            } else if instruction == "DIRECT" {
                return wire1_value;
            }

            if let Some(wire2) = &instructions.wire2 {
                let wire2_value: i32;
                if is_number(&wire2) {
                    wire2_value = wire2.parse().expect("Should be a number!");
                } else {
                    wire2_value = evaluate_wire(&wire2, hash_map);
                }
                if instruction == "AND" {
                    return wire1_value & wire2_value;
                } else if instruction == "RSHIFT" {
                    return wire1_value >> wire2_value;
                } else if instruction == "LSHIFT" {
                    return wire1_value << wire2_value;
                } else if instruction == "OR" {
                    return wire1_value | wire2_value;
                }
            }
        }
    }
    return 0;
}

fn is_number(str: &String) -> bool {
    return str.parse::<i32>().is_ok();
}

fn read_file() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

