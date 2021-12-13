use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}\n", day13a(INPUT));
    day13b(INPUT);
}

fn day13a(input: &str) -> usize {
    let (dots, folds) = process_input(input);

    fold_paper(&dots, folds[0]).len()
}

fn day13b(input: &str) {
    let (mut dots, folds) = process_input(input);

    for fold in folds {
        dots = fold_paper(&dots, fold)
    }

    let mut grid = [[false; 42]; 8];

    for dot in dots {
        grid[dot.1 as usize][dot.0 as usize] = true;
    }

    for row in grid {
        for cell in row {
            let sym = match cell {
                true => '#',
                false => ' ',
            };
            print!("{}", sym);
        }
        println!();
    }
}

fn fold_paper(dots: &HashSet<(i32, i32)>, fold: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut new_dots = HashSet::new();

    for dot in dots.into_iter() {
        let mut new_dot = (0, 0);

        if dot.0 > fold.0 {
            let dx = fold.0 - dot.0;
            new_dot.0 = fold.0 + dx;
        } else {
            new_dot.0 = dot.0;
        }

        if dot.1 > fold.1 {
            let dy = fold.1 - dot.1;
            new_dot.1 = fold.1 + dy;
        } else {
            new_dot.1 = dot.1;
        }

        new_dots.insert(new_dot);
    }

    new_dots
}

fn process_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(i32, i32)>) {
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

        let fold = if ori == "x" {
            (pos.parse::<i32>().unwrap(), i32::MAX)
        } else {
            (i32::MAX, pos.parse::<i32>().unwrap())
        };

        folds.push(fold);
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
