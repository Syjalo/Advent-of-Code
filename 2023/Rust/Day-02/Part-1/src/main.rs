use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Vec<Cube>>,
}

impl Game {
    fn new(id: i32, rounds: Vec<Vec<Cube>>) -> Self {
        Self { id, rounds }
    }

    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|round| {
            round.iter().all(|cube| match cube.color {
                CubeColor::Red => cube.count <= 12,
                CubeColor::Green => cube.count <= 13,
                CubeColor::Blue => cube.count <= 14,
            })
        })
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((_, game)) = game(s) {
            return Ok(game);
        }

        Err(())
    }
}

#[derive(Debug)]
struct Cube {
    count: i32,
    color: CubeColor,
}

impl Cube {
    fn new(count: i32, color: CubeColor) -> Self {
        Self { count, color }
    }
}

#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let sum = part_1(&content);
    println!("{sum}");
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = separated_pair(complete::i32, tag(" "), alpha1)(input)?;

    Ok((input, Cube::new(count, CubeColor::from_str(color).unwrap())))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, game_id) = preceded(tag("Game "), complete::i32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;

    Ok((input, Game::new(game_id, rounds)))
}

fn part_1(content: &str) -> i32 {
    content
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let sum = super::part_1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(sum, 8);
    }
}
