use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day15a(INPUT));
    println!("{}", day15b(INPUT));
}

fn day15a(input: &str) -> i16 {
    let cave = process_input(input);

    a_star((cave.len() as i16 - 1, cave.len() as i16 - 1), &cave)
}

fn day15b(input: &str) -> i16 {
    let cave = expand_cave(process_input(input));

    a_star((cave.len() as i16 - 1, cave.len() as i16 - 1), &cave)
}

// h(n) = Manhattan distance to goal
fn a_star(goal: (i16, i16), grid: &Vec<Vec<u8>>) -> i16 {
    let mut fringe = BinaryHeap::from([(0, 0, 0)]);
    let mut dist = vec![vec![i16::MAX; grid.len()]; grid.len()];
    dist[0][0] = 0;

    while let Some(curr) = fringe.pop() {
        let current = (curr.1, curr.2);

        if current == goal {
            return -curr.0;
        }

        for n in neighbors(current, grid) {
            let tent_g_score = dist[current.0 as usize][current.1 as usize]
                + grid[n.0 as usize][n.1 as usize] as i16;

            if tent_g_score < dist[n.0 as usize][n.1 as usize] {
                let f_n = tent_g_score + (goal.0 - n.0) + (goal.1 - n.1);
                fringe.push((-f_n, n.0, n.1));

                dist[n.0 as usize][n.1 as usize] = tent_g_score;
            }
        }
    }
    unreachable!()
}

fn expand_cave(cave: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..(5 * cave.len()))
        .map(|x| {
            (0..(5 * cave.len()))
                .map(|y| {
                    let x_lvl = (x / cave.len()) as u8;
                    let y_lvl = (y / cave.len()) as u8;

                    let risk = cave[x % cave.len()][y % cave.len()] + x_lvl + y_lvl;

                    if risk < 10 {
                        risk
                    } else {
                        risk - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn neighbors(current: (i16, i16), grid: &Vec<Vec<u8>>) -> Vec<(i16, i16)> {
    let mut neighbors = Vec::with_capacity(4);

    if current.0 != 0 {
        neighbors.push((current.0 - 1, current.1));
    }

    if current.1 != 0 {
        neighbors.push((current.0, current.1 - 1));
    }

    if current.0 != grid.len() as i16 - 1 {
        neighbors.push((current.0 + 1, current.1));
    }

    if current.1 != grid.len() as i16 - 1 {
        neighbors.push((current.0, current.1 + 1));
    }

    neighbors
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
