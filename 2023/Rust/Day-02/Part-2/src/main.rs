use std::{collections::BTreeMap, fs, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CubeColor {
    Blue,
    Green,
    Red,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(CubeColor::Blue),
            "green" => Ok(CubeColor::Green),
            "red" => Ok(CubeColor::Red),
            _ => Err(()),
        }
    }
}

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
        let (_, input) = line.split_once(": ").unwrap();
        let rounds = input.split("; ");
        let mut maxs: BTreeMap<CubeColor, i32> = BTreeMap::new();

        for round in rounds {
            let cubes = round.split(", ");

            for cube in cubes {
                let (count, color) = cube.split_once(" ").unwrap();
                let count: i32 = count.parse().unwrap();
                let color: CubeColor = color.parse().unwrap();

                if maxs.get(&color).unwrap_or(&i32::MIN) < &count {
                    maxs.insert(color, count);
                }
            }
        }

        let product: i32 = maxs.values().product();
        sum += product;
    }

    sum
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2() {
        let sum = super::part_2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(sum, 2286);
    }
}
