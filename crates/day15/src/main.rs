use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day15a(INPUT));
    println!("{}", day15b(INPUT));
}

fn day15a(input: &str) -> usize {
    let cave = process_input(input);

    a_star(
        (0, 0),
        (cave.len() as u16 - 1, cave.len() as u16 - 1),
        &cave,
    )
}

fn day15b(input: &str) -> usize {
    let cave = expand_cave(process_input(input));

    a_star(
        (0, 0),
        (cave.len() as u16 - 1, cave.len() as u16 - 1),
        &cave,
    )
}

// h(n) = Manhattan distance to goal
fn a_star(start: (u16, u16), goal: (u16, u16), grid: &Vec<Vec<u8>>) -> usize {
    let mut fringe = BinaryHeap::from([Reverse(start)]);
    let mut in_fringe = HashSet::new();
    in_fringe.insert(start);

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
        in_fringe.remove(&current);

        for n in neighbors(current, grid) {
            let tent_g_score =
                g_score.get(&current).unwrap() + grid[n.0 as usize][n.1 as usize] as u16;

            if tent_g_score < *g_score.get(&n).unwrap_or(&u16::MAX) {
                came_from.insert(n, current);
                g_score.insert(n, tent_g_score);
                f_score.insert(n, tent_g_score + (goal.0 - n.0) + (goal.1 - n.1));

                if !in_fringe.contains(&n) {
                    fringe.push(Reverse(n));
                    in_fringe.insert(n);
                }
            }
        }
    }
    0
}

fn expand_cave(cave: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_cave: Vec<Vec<u8>> = Vec::with_capacity(5 * cave.len());

    for _ in 0..cave.len() * 5 {
        new_cave.push(vec![0; cave.len() * 5]);
    }

    for (i, col) in cave.iter().enumerate() {
        for (j, v) in col.iter().enumerate() {
            new_cave[i][j] = *v;
        }
    }

    for (i, row) in cave.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            for s_o in 1..5 {
                let risk_out = *col + 1 * s_o as u8;
                let risk_out = if (risk_out % 9) == 0 { 9 } else { risk_out % 9 };
                new_cave[i][(j + (row.len() * s_o))] = risk_out;
                new_cave[(i + (row.len() * s_o))][j] = risk_out;

                for s_i in 1..5 {
                    let risk_in = *col + 1 * s_o as u8 + 1 * s_i as u8;
                    let risk_in = if (risk_in % 9) == 0 { 9 } else { risk_in % 9 };
                    new_cave[(i + (row.len() * s_o))][(j + (row.len() * s_i))] = risk_in;
                }
            }
        }
    }

    new_cave
}

fn neighbors(current: (u16, u16), grid: &Vec<Vec<u8>>) -> Vec<(u16, u16)> {
    let mut neighbors = Vec::new();

    if current.0 != 0 {
        neighbors.push((current.0 - 1, current.1));
    }

    if current.1 != 0 {
        neighbors.push((current.0, current.1 - 1));
    }

    if current.0 != grid.len() as u16 - 1 {
        neighbors.push((current.0 + 1, current.1));
    }

    if current.1 != grid.len() as u16 - 1 {
        neighbors.push((current.0, current.1 + 1));
    }

    neighbors
}

fn reconstruct_path(
    came_from: HashMap<(u16, u16), (u16, u16)>,
    current: (u16, u16),
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
        .fold(0, |acc, p| acc + grid[p.0 as usize][p.1 as usize] as usize)
        - grid[0][0] as usize
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

    #[test]
    fn part_b() {
        let expected = 315;
        let actual = day15b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
