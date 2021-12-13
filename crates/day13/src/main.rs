use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}\n", day13a(INPUT));
    day13b(INPUT);
}

fn day13a(input: &str) -> usize {
    let (dots, folds) = process_input(input);

    fold_paper(dots, folds[0]).len()
}

fn day13b(input: &str) {
    let (mut dots, folds) = process_input(input);

    for fold in folds {
        dots = fold_paper(dots, fold);
    }

    let mut grid = [[false; 42]; 8];

    for dot in dots {
        grid[dot.1 as usize][dot.0 as usize] = true;
    }

    for row in grid {
        for cell in row {
            let sym = match cell {
                true => '█',
                false => ' ',
            };
            print!("{}", sym);
        }
        println!();
    }
}

fn fold_paper(dots: HashSet<(i32, i32)>, fold: (char, i32)) -> HashSet<(i32, i32)> {
    dots.into_iter()
        .map(|dot| match fold.0 {
            'x' => {
                if dot.0 < fold.1 {
                    dot
                } else {
                    let dx = fold.1 - dot.0;
                    (fold.1 + dx, dot.1)
                }
            }
            _ => {
                if dot.1 < fold.1 {
                    dot
                } else {
                    let dy = fold.1 - dot.1;
                    (dot.0, fold.1 + dy)
                }
            }
        })
        .collect::<HashSet<_>>()
}

fn process_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(char, i32)>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    let (s1, s2) = input.split_once("\n\n").unwrap();

    s1.lines().for_each(|line| {
        let (xs, ys) = line.split_once(",").unwrap();

        dots.insert((xs.parse::<i32>().unwrap(), ys.parse::<i32>().unwrap()));
    });

    s2.lines().for_each(|line| {
        let (_, fold) = line.split_once(" along ").unwrap();
        let (ori, pos) = fold.split_once("=").unwrap();
        folds.push((ori.chars().next().unwrap(), pos.parse::<i32>().unwrap()));
    });

    (dots, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 17;
        let actual = day13a(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
