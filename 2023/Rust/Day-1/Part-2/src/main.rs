use std::fs;

const DIGITS_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let mut sum = 0;

    for line in lines {
        let mut digits: Vec<i32> = Vec::new();

        for (index, character) in line.chars().enumerate() {
            if let Some(digit) = character.to_digit(10) {
                digits.push(digit as i32);
                continue;
            }

            let subline = line.get(index..).unwrap();

            for (index, digit_name) in DIGITS_NAMES.iter().enumerate() {
                if subline.starts_with(digit_name) {
                    digits.push(index as i32 + 1);
                }
            }
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        sum += first_digit * 10 + last_digit;
    }

    println!("{sum}");
}
