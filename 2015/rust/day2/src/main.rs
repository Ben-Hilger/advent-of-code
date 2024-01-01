use std::fs::read_to_string;
use std::cmp::min;

struct Present {
    length: i32,
    width: i32,
    height: i32
}

impl Present {
    fn get_volume(&self) -> i32 {
        return self.length * self.width * self.height;
    }

    fn get_smallest_perimeter(&self) -> i32 {
        let current_min = min(self.length*2 + self.width*2, 
                                self.length*2 + self.height*2);
        return min(current_min, self.width*2 + self.height*2);
    }

    fn get_surface_area(&self) -> i32 {
        return 2*self.length*self.width + 
                2*self.width*self.height + 
                2*self.height*self.length;
    }

    fn get_slack(&self) -> i32 {
        let current_min = min(self.length*self.width, self.width*self.height);
        return min(current_min, self.length*self.height);
    }
}

fn main() {
    part1();
    part2();
}

fn read_file() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn part2() {
    let mut feet_of_ribbon_sum = 0;

    for line in read_file() {
        let dimensions: Vec<i32> = line.split("x")
                                .map(|s| s.to_string().parse().unwrap())
                                .collect();
        let present = build_present_from_split(dimensions);
        let volume = present.get_volume();
        let smallest_perimeter = present.get_smallest_perimeter();
        feet_of_ribbon_sum += volume + smallest_perimeter;
    }

    println!("The total amount of ribbon needed is: {}", feet_of_ribbon_sum);
}

fn part1() {
    let mut surface_area_sum = 0;

    for line in read_file() {
        let dimensions: Vec<i32> = line.split("x")
                                .map(|s| s.to_string().parse().unwrap())
                                .collect();
        let present = build_present_from_split(dimensions);
        let surface_area = present.get_surface_area();
        let slack = present.get_slack();
        surface_area_sum += surface_area + slack;
    }

    println!("The total amount of surface area needed is: {}", surface_area_sum);
}

fn build_present_from_split(split: Vec<i32>) -> Present {
    let length: i32 = split[0];
    let width: i32 = split[1];
    let height: i32 = split[2];
    return Present {
        length: length,
        width: width,
        height: height
    };
}
