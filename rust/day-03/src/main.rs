use itertools::Itertools;

const INPUT: &str = include_str!("../data/input.txt");

#[derive(Debug)]
struct Bitcounts {
    ones: usize,
    zeros: usize,
}

fn power_consumption(input: &Vec<Vec<bool>>) -> usize {
    let gamma: Vec<bool> = (0..input[0].len())
        .map(|col| input.iter().filter(|row| row[col]).count())
        .map(|ones| ones > input.len() / 2)
        .collect();

    let epsilon: Vec<bool> = gamma.iter().map(|x| !x).collect();

    bitvector_to_usize(gamma) * bitvector_to_usize(epsilon)
}

fn bitvector_to_usize(v: Vec<bool>) -> usize {
    usize::from_str_radix(v.iter().map(|&el| el as usize).join("").as_str(), 2).unwrap()
}

fn count_bits_in_column(input: &Vec<&str>, column: usize) -> Bitcounts {
    let bits = input.iter().map(|line| line.chars().nth(column).unwrap());

    let ones = bits.filter(|&c| c == '1').count();
    let zeros = input.len() - ones;
    Bitcounts { ones, zeros }
}

fn oxygen_winner(bitcounts: Bitcounts) -> char {
    if bitcounts.ones >= bitcounts.zeros {
        '1'
    } else {
        '0'
    }
}

fn co2_winner(bitcounts: Bitcounts) -> char {
    if bitcounts.ones >= bitcounts.zeros {
        '0'
    } else {
        '1'
    }
}

fn safety_rating(input: &Vec<&str>, winner: &dyn Fn(Bitcounts) -> char) -> usize {
    let n_columns = input[0].len();

    let mut lines = input.clone();

    for col in 0..n_columns {
        let bitcounts = count_bits_in_column(&lines, col);
        let winner = winner(bitcounts);

        lines = lines
            .into_iter()
            .filter(|&line| line.chars().nth(col).unwrap() == winner)
            .collect();
        if lines.len() == 1 {
            break;
        }
    }
    usize::from_str_radix(&lines[0], 2).unwrap()
}

fn life_support_rating(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let oxygen_generator_rating = safety_rating(&lines, &oxygen_winner);
    let co2_scrubber_rating = safety_rating(&lines, &co2_winner);
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let lines: Vec<Vec<bool>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();

    println!("power consumption: {}", power_consumption(&lines));
    println!("life support rating: {}", life_support_rating(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_power_consumption() {
        let lines: Vec<Vec<bool>> = SAMPLE
            .lines()
            .map(|line| line.chars().map(|c| c == '1').collect())
            .collect();
        assert_eq!(power_consumption(&lines), 198)
    }

    #[test]
    fn test_life_support_rating() {
        assert_eq!(life_support_rating(SAMPLE), 230)
    }
}
