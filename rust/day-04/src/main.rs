use ndarray::{Array3, ArrayBase, Axis, Dim, ViewRepr};
use regex::Regex;

const INPUT: &str = include_str!("../data/test.txt");
const GRID_SIZE: usize = 5;

type Cell = (bool, i32);
type Board = Vec<Vec<Cell>>;

fn check_rows(board: &Board) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|&(marked, _)| marked))
}

fn check_columns(board: &Board) -> bool {
    board
        .iter()
        .fold(vec![true, true, true, true, true], |acc, row| {
            let row_marks: Vec<bool> = row.iter().map(|&(marked, _)| marked).collect();
            row_marks
                .iter()
                .zip(acc.iter())
                .map(|(&r, &a)| r & a)
                .collect()
        })
        .iter()
        .any(|&col| col)
}

fn get_scores(input: &str) -> Vec<i32> {
    let (draws_string, boards_string) = input.split_once("\n\n").unwrap();

    let draws: Vec<i32> = draws_string
        .split(',')
        .map(|draw| draw.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = boards_string
        .split("\n\n")
        .map(|board| {
            board
                .split('\n')
                .map(|row| {
                    row.split_whitespace()
                        .map(|cell| (false, cell.parse::<i32>().unwrap()))
                        .collect()
                })
                .collect()
        })
        .collect();

    println!("{:?}", boards);

    for draw in draws {
        boards.iter_mut().map(|board| {
            board.iter_mut().map(|row| {
                row.iter_mut().for_each(|&mut (mut marked, value)| {
                    if value == draw {
                        marked = true
                    }
                })
            })
        });
    }

    vec![0, 1]
    // let num: Regex = Regex::new(r"\d+").unwrap();
    // let flattened_boards: Vec<&str> = num.find_iter(boards).map(|x| x.as_str()).collect();

    // let shape = (
    //     &flattened_boards.len() / (GRID_SIZE * GRID_SIZE),
    //     GRID_SIZE,
    //     GRID_SIZE,
    // );

    // let mut arr = Array3::from_shape_vec(shape, flattened_boards).unwrap();
    // let mut scores: Vec<i32> = Vec::new();

    // for n in numbers {
    //     arr.mapv_inplace(|a| if a == n { "x" } else { a });

    //     let winners: Vec<(usize, Board)> = arr
    //         .outer_iter()
    //         .enumerate()
    //         .filter(|(_, board)| check_rows(board) || check_columns(board))
    //         .collect::<Vec<_>>();

    //     let (mut win_indices, win_boards): (Vec<_>, Vec<_>) = winners.into_iter().unzip();

    //     let points: Vec<i32> = win_boards
    //         .iter()
    //         .map(|winner| {
    //             winner
    //                 .iter()
    //                 .filter(|&&cell| cell != "x")
    //                 .map(|&x| x.parse::<i32>().unwrap())
    //                 .fold(0, |a, b| a + b)
    //         })
    //         .map(|x| x * n.parse::<i32>().unwrap())
    //         .collect();
    //     scores.extend(points);

    //     win_indices.sort_by(|a, b| b.cmp(a));

    //     for idx in win_indices {
    //         arr.remove_index(Axis(0), idx);
    //     }
    // }
    // scores
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
