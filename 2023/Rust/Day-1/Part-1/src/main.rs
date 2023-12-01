use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let mut sum = 0;

    for line in lines {
        let mut digits: Vec<i32> = Vec::new();

        for character in line.chars() {
            if let Some(digit) = character.to_digit(10) {
                digits.push(digit as i32);
            }
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        sum += first_digit * 10 + last_digit;
    }

    println!("{sum}");
}
