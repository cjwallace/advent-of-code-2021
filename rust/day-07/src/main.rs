const INPUT: &str = include_str!("../data/input.txt");

fn constant_rate_fuel(input: &Vec<i32>) -> i32 {
    let median = input[input.len() / 2];
    input.iter().map(|n| (n - median).abs()).sum()
}

fn crab_rate_fuel(input: &Vec<i32>) -> i32 {
    let mean = input.iter().sum::<i32>() as f32 / input.len() as f32;

    let fuel = |mean: i32| {
        input
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
    let mut input: Vec<i32> = INPUT
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    input.sort();

    println!("Part one fuel: {:?}", constant_rate_fuel(&input));
    println!("Part two fuel: {:?}", crab_rate_fuel(&input));
}

#[cfg(test)]
mod tests {

    use super::{constant_rate_fuel, crab_rate_fuel};

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_constant_rate_fuel() {
        let mut input: Vec<i32> = SAMPLE
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        input.sort();
        assert_eq!(constant_rate_fuel(&input), 37);
    }

    #[test]
    fn test_crab_rate_fuel() {
        let mut input: Vec<i32> = SAMPLE
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        input.sort();
        assert_eq!(crab_rate_fuel(&input), 168);
    }
}
