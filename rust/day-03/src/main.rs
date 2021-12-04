use itertools::Itertools;

const INPUT: &str = include_str!("../data/input.txt");

fn parse_lines(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn power_consumption(input: &Vec<Vec<bool>>) -> usize {
    let gamma: Vec<bool> = (0..input[0].len())
        .map(|col| input.iter().filter(|row| row[col]).count())
        .map(|ones| ones > input.len() / 2)
        .collect();

    let epsilon: Vec<bool> = gamma.iter().map(|x| !x).collect();

    bitvector_to_usize(&gamma) * bitvector_to_usize(&epsilon)
}

fn bitvector_to_usize(v: &Vec<bool>) -> usize {
    usize::from_str_radix(v.iter().map(|&el| el as usize).join("").as_str(), 2).unwrap()
}

fn oxygen_winner(ones: usize, zeros: usize) -> bool {
    ones >= zeros
}

fn co2_winner(ones: usize, zeros: usize) -> bool {
    ones < zeros
}

fn count_bits_in_column(input: &Vec<Vec<bool>>, column: usize) -> (usize, usize) {
    let ones = input
        .iter()
        .map(|line| line[column])
        .filter(|&bit| bit)
        .count();
    let zeros = input.len() - ones;
    (ones, zeros)
}

fn safety_rating(input: &Vec<Vec<bool>>, winner: &dyn Fn(usize, usize) -> bool) -> usize {
    let n_columns = input[0].len();

    let mut lines = input.clone();

    for col in 0..n_columns {
        let (ones, zeros) = count_bits_in_column(&lines, col);
        let winner = winner(ones, zeros);

        lines = lines
            .into_iter()
            .filter(|line| line[col] == winner)
            .collect();
        if lines.len() == 1 {
            break;
        }
    }

    bitvector_to_usize(&lines[0])
}

fn life_support_rating(lines: &Vec<Vec<bool>>) -> usize {
    let oxygen_generator_rating = safety_rating(&lines, &oxygen_winner);
    let co2_scrubber_rating = safety_rating(&lines, &co2_winner);
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let lines = parse_lines(INPUT);

    println!("power consumption: {}", power_consumption(&lines));
    println!("life support rating: {}", life_support_rating(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_power_consumption() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(power_consumption(&lines), 198)
    }

    #[test]
    fn test_life_support_rating() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(life_support_rating(&lines), 230)
    }
}
