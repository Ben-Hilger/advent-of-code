use md5;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = "yzbqklnj";

    let mut hash_number = 1;
    let mut hash_input = format!("{}{}", input, hash_number);
    let mut hash = md5::compute(hash_input);
    while !has_at_least_five_leading_zeros(format!("{:x}", hash)) {
        hash_number += 1;
        hash_input = format!("{}{}", input, hash_number);
        hash = md5::compute(hash_input);
    }

    println!("The smallest positive number to get the correct hash is: {}", hash_number);
}

fn part2() {
    let input = "yzbqklnj";

    let mut hash_number = 1;
    let mut hash_input = format!("{}{}", input, hash_number);
    let mut hash = md5::compute(hash_input);
    while !has_at_least_six_leading_zeros(format!("{:x}", hash)) {
        hash_number += 1;
        hash_input = format!("{}{}", input, hash_number);
        hash = md5::compute(hash_input);
    }

    println!("The smallest positive number to get the correct hash is: {}", hash_number);

}

fn has_at_least_five_leading_zeros(hash: String) -> bool {
    return hash.starts_with("00000");    
}

fn has_at_least_six_leading_zeros(hash: String) -> bool {
    return hash.starts_with("000000");
}
