use ndarray::{Array3, Axis};
use regex::Regex;

const INPUT: &str = include_str!("../data/test.txt");

fn get_scores(input: &str) -> Vec<i32> {
    let mut split = input.splitn(2, "\n\n");
    let numbers: Vec<&str> = split.next().unwrap().split(',').collect();
    let grids: &str = split.next().unwrap();

    let num: Regex = Regex::new(r"\d+").unwrap();
    let flat: Vec<&str> = num.find_iter(grids).map(|x| x.as_str()).collect();

    let grid_size = 5;

    let shape = (&flat.len() / (grid_size * grid_size), grid_size, grid_size);

    let mut arr = Array3::from_shape_vec(shape, flat).unwrap();
    let mut scores: Vec<i32> = Vec::new();

    for n in numbers {
        arr.mapv_inplace(|a| if a == n { "x" } else { a });

        let winners = arr
            .outer_iter()
            .enumerate()
            .filter(|(_, grid)| {
                (grid
                    .columns()
                    .into_iter()
                    .any(|col| col.iter().all(|&c| c == "x")))
                    || (grid
                        .rows()
                        .into_iter()
                        .any(|row| row.iter().all(|&r| r == "x")))
            })
            .collect::<Vec<_>>();

        let (mut win_indices, win_grids): (Vec<_>, Vec<_>) = winners.into_iter().unzip();

        let points: Vec<i32> = win_grids
            .iter()
            .map(|winner| {
                winner
                    .iter()
                    .filter(|&&cell| cell != "x")
                    .map(|&x| x.parse::<i32>().unwrap())
                    .fold(0, |a, b| a + b)
            })
            .map(|x| x * n.parse::<i32>().unwrap())
            .collect();
        scores.extend(points);

        win_indices.sort_by(|a, b| b.cmp(a));

        for idx in win_indices {
            arr.remove_index(Axis(0), idx);
        }
    }
    scores
}

fn main() {
    let scores = get_scores(INPUT);
    println!("first winner score: {}", scores.first().unwrap());
    println!("final winner score: {}", scores.last().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::get_scores;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_get_scores() {
        let scores = get_scores(SAMPLE);
        assert_eq!(scores.first().unwrap(), &4512i32);
        assert_eq!(scores.last().unwrap(), &1924i32);
    }
}
