use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day15a(INPUT));
}

fn day15a(input: &str) -> usize {
    let cave = process_input(input);

    a_star((0, 0), (cave.len() - 1, cave.len() - 1), &cave)
}

// h(n) = Manhattan distance to goal
fn a_star(start: (usize, usize), goal: (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    let mut fringe = BinaryHeap::from([Reverse(start)]);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, (goal.0 - start.0) + (goal.1 - start.1));

    while fringe.len() != 0 {
        let current = fringe.peek().unwrap().0;

        if current == goal {
            return reconstruct_path(came_from, current, grid);
        }

        fringe.pop();

        for n in neighbors(current, grid) {
            let tent_g_score = g_score.get(&current).unwrap() + grid[n.0][n.1] as usize;

            if tent_g_score < *g_score.get(&n).unwrap_or(&usize::MAX) {
                came_from.insert(n, current);
                g_score.insert(n, tent_g_score);
                f_score.insert(n, tent_g_score + (goal.0 - n.0) + (goal.1 - n.1));

                match fringe.iter().find(|&&x| x.0 == n) {
                    None => {
                        fringe.push(Reverse(n));
                    }
                    Some(_) => (),
                }
            }
        }
    }

    0
}

fn neighbors(current: (usize, usize), grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if current.0 != 0 {
        neighbors.push((current.0 - 1, current.1));
    }

    if current.1 != 0 {
        neighbors.push((current.0, current.1 - 1));
    }

    if current.0 != grid.len() - 1 {
        neighbors.push((current.0 + 1, current.1));
    }

    if current.1 != grid.len() - 1 {
        neighbors.push((current.0, current.1 + 1));
    }

    neighbors
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
    grid: &Vec<Vec<u8>>,
) -> usize {
    let mut total_path = Vec::from([current]);

    let mut tmp = current;

    while came_from.contains_key(&tmp) {
        tmp = *came_from.get(&tmp).unwrap();
        total_path.push(tmp);
    }

    total_path
        .into_iter()
        .fold(0, |acc, p| acc + grid[p.0][p.1] as usize) - grid[0][0] as usize
}

fn process_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 40;
        let actual = day15a(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
