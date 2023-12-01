use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let mut sum: i32 = 0;

    for line in lines {
        let mut numbers: Vec<i32> = Vec::new();

        for character in line.chars() {
            if let Some(number) = character.to_digit(10) {
                numbers.push(number as i32);
            }
        }

        let first_number = numbers.first().unwrap();
        let last_number = numbers.last().unwrap();
        sum += first_number * 10 + last_number;
    }

    println!("{sum}");
}
