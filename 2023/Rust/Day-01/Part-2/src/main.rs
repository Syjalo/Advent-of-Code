use std::fs;

const DIGITS_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let sum = part_2(&content);
    println!("{sum}");
}

fn part_2(content: &str) -> i32 {
    let lines = content
        .lines()
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

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2() {
        let sum = super::part_2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(sum, 281);
    }
}
