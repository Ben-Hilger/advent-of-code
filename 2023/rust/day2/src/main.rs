use std::fs::read_to_string;
use std::cmp::max;

fn main() {
    println!("Hello, world!");
    part1();
    part2();
}

fn part2() {
    let lines = read_input();

    let mut sum: i32 = 0;

    for line in lines {
        let game_summary_info = line.split(":").collect::<Vec<_>>();
        let game_info = game_summary_info[1];
        let games = game_info.split(";").collect::<Vec<_>>();
        
        let mut max_blue = 1;
        let mut max_green = 1;
        let mut max_red = 1;
        
        for game in games {

            let dice_info = game.split(",").collect::<Vec<_>>();
            for dice in dice_info {
                let dice_split = dice.trim().split(" ").collect::<Vec<_>>();
                let number_of_dice: i32 = dice_split[0].parse().unwrap();
                let dice_color= dice_split[1].trim();
                
                if dice_color == "blue" {
                    max_blue = max(number_of_dice, max_blue);
                } else if dice_color == "red" {
                    max_red = max(number_of_dice, max_red);
                } else if dice_color == "green" {
                    max_green = max(number_of_dice, max_green);
                }
            }
        }
        println!("{} {} {} {}", game_summary_info[0], max_blue, max_red, max_green);
        sum += max(max_blue, 1) * max(max_red, 1) * max(max_green, 1);
    }

    println!("The sum of min number of dice are: {}", sum);
}

fn part1() {
    let lines = read_input();

    let green_max = 13;
    let blue_max = 14;
    let red_max = 12;

    let mut sum: i32 = 0;

    for line in lines {
        let game_summary_info = line.split(":").collect::<Vec<_>>();
        let game_id: i32 = game_summary_info[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let game_info = game_summary_info[1];
        let games = game_info.split(";").collect::<Vec<_>>();
        
        let mut game_valid = true;

        for game in games {
            let dice_info = game.split(",").collect::<Vec<_>>();
            for dice in dice_info {
                let dice_split = dice.trim().split(" ").collect::<Vec<_>>();
                let number_of_dice: i32 = dice_split[0].parse().unwrap();
                let dice_color= dice_split[1];
                if (dice_color == "green" && number_of_dice > green_max) ||
                    (dice_color == "blue" && number_of_dice > blue_max) ||
                    (dice_color == "red" && number_of_dice > red_max) {
                    game_valid = false;
                }
            }

        }

        if game_valid {
            sum += game_id;
        }
    }

    println!("The sum of valid games are: {}", sum);
}

fn read_input() -> Vec<String> {
    return read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();
}
