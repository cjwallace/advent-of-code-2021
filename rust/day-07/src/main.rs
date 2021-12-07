const INPUT: &str = include_str!("../data/input.txt");

fn parse_positions(input: &str) -> Vec<i32> {
    let mut positions: Vec<i32> = input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    positions.sort();
    positions
}

fn constant_rate_fuel(positions: &Vec<i32>) -> i32 {
    let median = positions[positions.len() / 2];
    positions.iter().map(|n| (n - median).abs()).sum()
}

fn crab_rate_fuel(positions: &Vec<i32>) -> i32 {
    let mean = positions.iter().sum::<i32>() as f32 / positions.len() as f32;

    let fuel = |mean: i32| {
        positions
            .iter()
            .map(|n| {
                let dist = (n - mean).abs();
                dist * (dist + 1) / 2
            })
            .sum()
    };

    i32::min(fuel(mean.floor() as i32), fuel(mean.ceil() as i32))
}

fn main() {
    let positions = parse_positions(INPUT);
    println!("Part one fuel: {:?}", constant_rate_fuel(&positions));
    println!("Part two fuel: {:?}", crab_rate_fuel(&positions));
}

#[cfg(test)]
mod tests {

    use super::{constant_rate_fuel, crab_rate_fuel, parse_positions};

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_constant_rate_fuel() {
        let positions = parse_positions(SAMPLE);
        assert_eq!(constant_rate_fuel(&positions), 37);
    }

    #[test]
    fn test_crab_rate_fuel() {
        let positions = parse_positions(SAMPLE);
        assert_eq!(crab_rate_fuel(&positions), 168);
    }
}
