const INPUT: &str = include_str!("../data/input.txt");

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

type Line = (Coord, Coord);

fn parse_coord(input: &str) -> Coord {
    let (x, y) = input.split_once(",").unwrap();
    Coord {
        x: x.parse::<i32>().unwrap(),
        y: y.parse::<i32>().unwrap(),
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(to, from)| (parse_coord(to), parse_coord(from)))
        .collect()
}

fn mark_lines(map: &[Vec<i32>], lines: &[Line]) -> Vec<Vec<i32>> {
    let mut map = map.to_owned();
    let points: Vec<Coord> = lines
        .iter()
        .map(|(from, to)| {
            let dx = i32::signum(to.x - from.x);
            let dy = i32::signum(to.y - from.y);

            let n_points = i32::max(i32::abs(from.x - to.x), i32::abs(from.y - to.y));

            let coords: Vec<Coord> = (0..=n_points)
                .map(|n| Coord {
                    x: from.x + n * dx,
                    y: from.y + n * dy,
                })
                .collect();
            coords
        })
        .flatten()
        .collect();

    points
        .iter()
        .for_each(|p| map[p.y as usize][p.x as usize] += 1);

    map
}

fn count_vents(vents: Vec<Vec<i32>>) -> i32 {
    vents.iter().flatten().filter(|&&x| x > 1).count() as i32
}

fn initialize_vent_map(instructions: &[Line]) -> Vec<Vec<i32>> {
    let (width, height) = instructions
        .iter()
        .fold((0, 0), |(width, height), (from, to)| {
            (
                i32::max(width, i32::max(from.x, to.x)),
                i32::max(height, i32::max(from.y, to.y)),
            )
        });

    vec![vec![0; (width + 1) as usize]; (height + 1) as usize]
}

fn filter_diagonals(lines: &[Line]) -> Vec<Line> {
    lines
        .iter()
        .filter(|(from, to)| (from.x == to.x) | (from.y == to.y))
        .cloned()
        .collect()
}

fn main() {
    let lines = parse_lines(INPUT);
    let non_diagonals: Vec<Line> = filter_diagonals(&lines);

    let empty_map = initialize_vent_map(&lines);

    let straight_vents = mark_lines(&empty_map, &non_diagonals);
    let all_vents = mark_lines(&empty_map, &lines);

    println!("non-diagonal vents: {:?}", count_vents(straight_vents));
    println!("all vents: {:?}", count_vents(all_vents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../data/test.txt");

    #[test]
    fn test_parse_coord() {
        let input = "123,42";
        let coord = Coord { x: 123, y: 42 };
        assert_eq!(parse_coord(input), coord)
    }

    #[test]
    fn test_parse_lines() {
        let input = "0,0 -> 1,1\n7,6 -> 1,3";
        let lines = vec![
            (Coord { x: 0, y: 0 }, Coord { x: 1, y: 1 }),
            (Coord { x: 7, y: 6 }, Coord { x: 1, y: 3 }),
        ];
        assert_eq!(parse_lines(input), lines);
    }

    #[test]
    fn functional_test() {
        let lines = parse_lines(SAMPLE);
        let non_diagonals: Vec<Line> = filter_diagonals(&lines);

        let empty_map = initialize_vent_map(&lines);

        let straight_vents = mark_lines(&empty_map, &non_diagonals);
        let all_vents = mark_lines(&empty_map, &lines);

        assert_eq!(count_vents(straight_vents), 5);
        assert_eq!(count_vents(all_vents), 12);
    }
}
