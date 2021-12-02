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

#[derive(Debug, PartialEq)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    n: i32,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| parse_instruction(line)).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let mut iter = line.splitn(2, ' ');
    let direction = Direction::from_str(iter.next().unwrap()).unwrap();
    let n = iter.next().unwrap().parse::<i32>().unwrap();

    Instruction { direction, n }
}

fn part_one(instructions: &Vec<Instruction>) -> Submarine {
    // Could take a Submarine as an argument and parse into a tuple
    // for easy manipulation, if we wanted to start from an arbitrary point
    let (horizontal, depth, aim) =
        instructions
            .iter()
            .fold((0, 0, 0), |(h, d, a), i| match i.direction {
                Direction::Forward => (h + i.n, d, a),
                Direction::Up => (h, d - i.n, a),
                Direction::Down => (h, d + i.n, a),
            });

    Submarine {
        horizontal,
        depth,
        aim,
    }
}

fn part_two(instructions: &Vec<Instruction>) -> Submarine {
    // Exactly as part_one, with different rules applied in the match
    let (horizontal, depth, aim) =
        instructions
            .iter()
            .fold((0, 0, 0), |(h, d, a), i| match i.direction {
                Direction::Forward => (h + i.n, d + a * i.n, a),
                Direction::Up => (h, d, a - i.n),
                Direction::Down => (h, d, a + i.n),
            });

    Submarine {
        horizontal,
        depth,
        aim,
    }
}

fn main() {
    let instructions = parse_instructions(INPUT);
    let first_submarine = part_one(&instructions);
    println!(
        "Part one:\n\t submarine state: {:?}\n\t horizontal * depth: {}",
        first_submarine,
        first_submarine.horizontal * first_submarine.depth
    );
    let second_submarine = part_two(&instructions);
    println!(
        "Part two:\n\t submarine state: {:?}\n\t horizontal * depth: {}",
        second_submarine,
        second_submarine.horizontal * second_submarine.depth
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_instruction("forward 3"),
            Instruction {
                direction: Direction::Forward,
                n: 3i32
            }
        );
        assert_eq!(
            parse_instruction("up 7"),
            Instruction {
                direction: Direction::Up,
                n: 7i32
            }
        );
        assert_eq!(
            parse_instruction("down 0"),
            Instruction {
                direction: Direction::Down,
                n: 0i32
            }
        );
    }

    #[test]
    fn test_part_one() {
        let instructions = parse_instructions(SAMPLE);
        assert_eq!(
            part_one(&instructions),
            Submarine {
                horizontal: 15,
                depth: 10,
                aim: 0
            }
        );
    }

    #[test]
    fn test_part_two() {
        let instructions = parse_instructions(SAMPLE);
        assert_eq!(
            part_two(&instructions),
            Submarine {
                horizontal: 15,
                depth: 60,
                aim: 10
            }
        )
    }
}
