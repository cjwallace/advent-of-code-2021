use ndarray::{Array3, ArrayView2, Axis};
use regex::Regex;

const INPUT: &str = include_str!("../data/input.txt");
const BOARD_SIZE: usize = 5;

fn parse_input(input: &str) -> (Vec<&str>, ndarray::Array3<&str>) {
    let mut split = input.splitn(2, "\n\n");
    let numbers: Vec<&str> = split.next().unwrap().split(',').collect();
    let boards: &str = split.next().unwrap();

    let num: Regex = Regex::new(r"\d+").unwrap();
    let flat_boards: Vec<&str> = num.find_iter(boards).map(|x| x.as_str()).collect();

    let shape = (
        &flat_boards.len() / (BOARD_SIZE * BOARD_SIZE),
        BOARD_SIZE,
        BOARD_SIZE,
    );

    // stack all boards into single 3d array
    let arr = Array3::from_shape_vec(shape, flat_boards).unwrap();

    (numbers, arr)
}

fn check_columns(board: &ArrayView2<&str>) -> bool {
    board
        .columns()
        .into_iter()
        .any(|col| col.iter().all(|&c| c == "x"))
}

fn check_rows(board: &ArrayView2<&str>) -> bool {
    board
        .rows()
        .into_iter()
        .any(|row| row.iter().all(|&r| r == "x"))
}

fn compute_scores(input: &str) -> Vec<i32> {
    let (draws, mut boards) = parse_input(input);

    let mut all_scores: Vec<i32> = Vec::new();

    for d in draws {
        // mark by replacing with 'x'
        boards.mapv_inplace(|a| if a == d { "x" } else { a });

        let winners = boards
            .outer_iter()
            .enumerate()
            .filter(|(_, board)| check_rows(board) || check_columns(board))
            .collect::<Vec<_>>();

        let (mut winning_board_indices, winning_boards): (Vec<_>, Vec<_>) =
            winners.into_iter().unzip();

        let scores: Vec<i32> = winning_boards
            .iter()
            .map(|winner| {
                winner
                    .iter()
                    .filter(|&&cell| cell != "x")
                    .map(|&x| x.parse::<i32>().unwrap())
                    .fold(0, |a, b| a + b)
            })
            .map(|x| x * d.parse::<i32>().unwrap())
            .collect();

        all_scores.extend(scores);

        // remove boards that have won
        winning_board_indices.sort_by(|a, b| b.cmp(a));

        for idx in winning_board_indices {
            boards.remove_index(Axis(0), idx);
        }
    }

    all_scores
}

fn main() {
    let scores = compute_scores(INPUT);
    println!("first winner score: {}", scores.first().unwrap());
    println!("final winner score: {}", scores.last().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::compute_scores;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_get_scores() {
        let scores = compute_scores(SAMPLE);
        assert_eq!(scores.first().unwrap(), &4512i32);
        assert_eq!(scores.last().unwrap(), &1924i32);
    }
}
