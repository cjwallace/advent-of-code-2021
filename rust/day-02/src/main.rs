use std::str::FromStr;

const INPUT: &str = include_str!("../data/input.txt");

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct Coordinates {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn parse_line(line: &str) -> (Direction, i32) {
    // let (direction, value) = line.split_at(line.find(' ').unwrap());
    let mut iter = line.splitn(2, ' ');
    let direction = iter.next().unwrap();
    let value = iter.next().unwrap();

    (
        Direction::from_str(direction).unwrap(),
        value.parse::<i32>().unwrap(),
    )
}

fn part_one(input_str: &str) -> i32 {
    let mut location = Coordinates {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    input_str
        .lines()
        .map(|line| parse_line(line))
        .for_each(|(direction, value)| match direction {
            Direction::Forward => {
                location.horizontal += value;
            }
            Direction::Down => {
                location.depth += value;
            }
            Direction::Up => {
                location.depth -= value;
                if location.depth < 0 {
                    location.depth = 0
                }
            }
        });

    println!(
        "horizontal: {}\ndepth: {}\nproduct: {}",
        location.horizontal,
        location.depth,
        location.horizontal * location.depth
    );

    return location.horizontal * location.depth;
}

fn part_two(input_str: &str) -> i32 {
    let mut location = Coordinates {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    input_str
        .lines()
        .map(|line| parse_line(line))
        .for_each(|(direction, value)| match direction {
            Direction::Forward => {
                location.horizontal += value;
                location.depth += value * location.aim;
            }
            Direction::Down => {
                location.aim += value;
            }
            Direction::Up => {
                location.aim -= value;
            }
        });

    println!(
        "horizontal: {}\ndepth: {}\nproduct: {}",
        location.horizontal,
        location.depth,
        location.horizontal * location.depth
    );

    return location.horizontal * location.depth;
}

fn main() {
    part_one(INPUT);
    part_two(INPUT);
}

#[cfg(test)]
mod tests {
    use super::{parse_line, part_one, part_two, Direction};

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("forward 3"), (Direction::Forward, 3i32));
        assert_eq!(parse_line("up 7"), (Direction::Up, 7i32));
        assert_eq!(parse_line("down 0"), (Direction::Down, 0i32));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 150);
        assert_eq!(part_one("down 2"), 0);
        assert_eq!(part_one("forward 1\ndown 1"), 1);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 900)
    }
}
