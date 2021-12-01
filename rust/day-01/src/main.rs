use std::fs;

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("That's not a valid file.")
        .split("\n")
        .filter_map(|h| h.parse::<i32>().ok())
        .collect()
}

fn count_depth_increases(depths: &Vec<i32>) -> i32 {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count() as i32
}

fn triplet_depths(depths: &Vec<i32>) -> Vec<i32> {
    depths
        .windows(3)
        .map(|triplet| triplet.iter().sum())
        .collect()
}

fn part_one(depths: &Vec<i32>) -> i32 {
    count_depth_increases(&depths)
}

fn part_two(depths: &Vec<i32>) -> i32 {
    count_depth_increases(&triplet_depths(&depths))
}

fn main() {
    let path = "data/input.txt";
    let input = read_input(path);
    println!("Part one: {}", part_one(&input));

    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::{count_depth_increases, read_input, triplet_depths};

    #[test]
    fn test_count_height_increases() {
        let input = read_input("data/test.txt");
        let num_increases = count_depth_increases(&input);
        assert_eq!(7, num_increases)
    }

    // part one test _is_ test_count_height_increases()

    #[test]
    fn test_triplet_depths() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 3, 2, 1];
        let depths = triplet_depths(&input);
        assert_eq!(vec![6, 9, 10, 9, 6], depths);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("data/test.txt");
        let depths = triplet_depths(&input);
        let num_increases = count_depth_increases(&depths);
        assert_eq!(5, num_increases);
    }
}
