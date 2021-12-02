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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn parse_instructions(input: &str) -> Vec<(Direction, i32)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (Direction, i32) {
    let mut iter = line.splitn(2, ' ');
    let direction = iter.next().unwrap();
    let value = iter.next().unwrap();

    (
        Direction::from_str(direction).unwrap(),
        value.parse::<i32>().unwrap(),
    )
}

fn part_one(instructions: &[(Direction, i32)], mut location: Submarine) -> Submarine {
    instructions
        .iter()
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

    location
}

fn part_two(instructions: &[(Direction, i32)], mut location: Submarine) -> Submarine {
    instructions
        .iter()
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

    location
}

fn print_answer(location: Submarine) {
    println!(
        "horizontal: {}\ndepth: {}\nproduct: {}",
        location.horizontal,
        location.depth,
        location.horizontal * location.depth
    );
}

fn main() {
    let submarine = Submarine {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    let instructions = parse_instructions(INPUT);
    let first_submarine = part_one(&instructions, submarine);
    println!("Part one:");
    print_answer(first_submarine);

    println!("\nPart two:");
    let second_submarine = part_two(&instructions, submarine);
    print_answer(second_submarine);
}

#[cfg(test)]
mod tests {
    use super::{parse_instructions, parse_line, part_one, part_two, Direction, Submarine};

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("forward 3"), (Direction::Forward, 3i32));
        assert_eq!(parse_line("up 7"), (Direction::Up, 7i32));
        assert_eq!(parse_line("down 0"), (Direction::Down, 0i32));
    }

    #[test]
    fn test_part_one() {
        let submarine = Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        let instructions = parse_instructions(SAMPLE);
        assert_eq!(
            part_one(&instructions, submarine),
            Submarine {
                horizontal: 15,
                depth: 10,
                aim: 0
            }
        );
    }

    #[test]
    fn test_part_two() {
        let submarine = Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        let instructions = parse_instructions(SAMPLE);
        assert_eq!(
            part_two(&instructions, submarine),
            Submarine {
                horizontal: 15,
                depth: 60,
                aim: 10
            }
        )
    }
}
