use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");
const WIDTH: i32 = 5;

fn main() {
    println!("{}", day04a(INPUT));
    println!("{}", day04b(INPUT));
}

fn day04a(input: &str) -> u64 {
    let (draws, mut boards) = process_input(input);

    let mut score = 0;

    'outer: for draw in draws {
        for board in boards.iter_mut() {
            mark_board(board, draw);

            if check_board(&board) {
                score = calculate_score(&board, draw);
                break 'outer;
            }
        }
    }

    score as u64
}

fn day04b(input: &str) -> u64 {
    let (draws, mut boards) = process_input(input);

    let mut score = 0;

    let mut won_boards = HashSet::new();
    let mut total_won = 0;
    let total_boards = boards.len();

    'outer: for draw in draws {
        for (i, board) in boards.iter_mut().enumerate() {
            mark_board(board, draw);

            if check_board(&board) {
                score = calculate_score(&board, draw);

                if !won_boards.contains(&i) {
                    total_won += 1;

                    if total_won == total_boards {
                        break 'outer;
                    }
                }

                won_boards.insert(i);
            }
        }
    }

    score as u64
}

#[inline]
fn mark_board(board: &mut Vec<i32>, num: i32) {
    for i in 0..WIDTH * WIDTH {
        if board[i as usize] == num {
            board[i as usize] = -1;
        }
    }
}

fn check_board(board: &Vec<i32>) -> bool {
    let mut cmplt_row = false;
    let mut cmplt_cols = [true, true, true, true, true];

    for (i, &sq) in board.iter().enumerate() {
        if i % 5 == 0 {
            if cmplt_row {
                return true;
            }

            cmplt_row = true;
        }

        if sq != -1 {
            cmplt_row = false;
            cmplt_cols[i % 5] = false;
        }
    }

    if cmplt_cols.contains(&true) {
        return true;
    }

    false
}

fn calculate_score(board: &Vec<i32>, last_num: i32) -> i32 {
    let score = board
        .iter()
        .filter(|&num| *num != -1)
        .fold(0, |acc, &num| acc + num);

    score * last_num
}

fn process_input(input: &str) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut inputs = input.split("\n\n").collect::<Vec<_>>();

    let draws = inputs
        .remove(0)
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards: Vec<Vec<i32>> = Vec::new();

    inputs.iter().for_each(|&s| {
        let mut tmp = Vec::new();

        s.split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .for_each(|n| tmp.push(n));

        boards.push(tmp);
    });

    (draws, boards)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 4512;
        let actual = day04a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 1924;
        let actual = day04b(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bingo_row() {
        let expected = true;

        let board = vec![
            -1, -1, -1, -1, -1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21,
        ];

        let actual = check_board(&board);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bingo_col() {
        let expected = true;

        let board = vec![
            -1, 0, 1, 2, 3, -1, 4, 5, 6, 7, -1, 8, 9, 10, 11, -1, 12, 13, 14, 15, -1, 16, 17, 18,
            19,
        ];

        let actual = check_board(&board);

        assert_eq!(expected, actual);
    }

    #[test]
    fn not_bingo() {
        let expected = false;

        let board = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24,
        ];

        let actual = check_board(&board);

        assert_eq!(expected, actual);
    }
}
