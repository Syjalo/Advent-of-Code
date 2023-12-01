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
        let mut numbers: Vec<i32> = Vec::new();

        for (index, character) in line.chars().enumerate() {
            if let Some(number) = character.to_digit(10) {
                numbers.push(number as i32);
                continue;
            }

            let subline = line.get(index..).unwrap();

            for (index, digit_name) in DIGITS_NAMES.iter().enumerate() {
                if subline.starts_with(digit_name) {
                    numbers.push(index as i32 + 1);
                }
            }
        }

        let first_number = numbers.first().unwrap();
        let last_number = numbers.last().unwrap();
        sum += first_number * 10 + last_number;
    }

    println!("{sum}");
}
