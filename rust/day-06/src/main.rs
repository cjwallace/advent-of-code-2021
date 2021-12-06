const INPUT: &str = include_str!("../data/input.txt");

fn shoal_size(input: &str, n_days: u64) -> u64 {
    let input_fish: Vec<u64> = input
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    /*
    represent shoal as vector of number of fish at
    index days until reproduction
    */
    let mut shoal = [0; 9];
    input_fish.iter().for_each(|&x| shoal[x as usize] += 1);

    for _ in 0..n_days {
        shoal.rotate_left(1);
        shoal[6] += shoal[8];
    }

    let shoal_size: u64 = shoal.iter().sum();
    shoal_size
}

fn main() {
    println!("shoal size at 80 days: {}", shoal_size(INPUT, 80));
    println!("shoal size at 256 days: {}", shoal_size(INPUT, 256));
}

#[cfg(test)]
mod tests {
    use super::shoal_size;
    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_shoal_size() {
        assert_eq!(shoal_size(SAMPLE, 18), 26);
        assert_eq!(shoal_size(SAMPLE, 80), 5934);
        assert_eq!(shoal_size(SAMPLE, 256), 26984457539);
    }
}
