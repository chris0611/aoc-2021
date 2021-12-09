use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day09a(INPUT));
    println!("{}", day09b(INPUT));
}

fn day09a(input: &str) -> usize {
    let heightmap = process_input(input);

    let mut result = 0;

    for (i, row) in heightmap.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if !is_lowest(&heightmap, (i, j)) {
                continue;
            }
            result += 1 + col as usize;
        }
    }

    result
}

fn day09b(input: &str) -> usize {
    let heightmap = process_input(input);

    let mut basins: Vec<usize> = Vec::new();

    for (i, row) in heightmap.iter().enumerate() {
        for j in 0..row.len() {
            if !is_lowest(&heightmap, (i, j)) {
                continue;
            }
            basins.push(basin_finder(&heightmap, (i, j)));
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn is_lowest(hm: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    let row = &hm[p.0];
    let col = hm[p.0][p.1];

    if col == 9 {
        return false;
    }
    if p.0 != 0 {
        if hm[p.0 - 1][p.1] <= col {
            return false;
        }
    }
    if p.1 != 0 {
        if row[p.1 - 1] <= col {
            return false;
        }
    }
    if p.0 != hm.len() - 1 {
        if hm[p.0 + 1][p.1] <= col {
            return false;
        }
    }
    if p.1 != row.len() - 1 {
        if row[p.1 + 1] <= col {
            return false;
        }
    }
    true
}

fn basin_finder(hm: &Vec<Vec<u8>>, init: (usize, usize)) -> usize {
    let mut found: HashSet<(usize, usize)> = HashSet::new();

    found.insert((init.0, init.1));
    basin_helper(hm, init, &mut found);

    found.len()
}

fn basin_helper(hm: &Vec<Vec<u8>>, p: (usize, usize), found: &mut HashSet<(usize, usize)>) {
    let curr = hm[p.0][p.1];

    if p.0 != 0 && !found.contains(&(p.0 - 1, p.1)) {
        if hm[p.0 - 1][p.1] != 9 && hm[p.0 - 1][p.1] > curr {
            found.insert((p.0 - 1, p.1));
            basin_helper(hm, (p.0 - 1, p.1), found);
        }
    }

    if p.1 != 0 && !found.contains(&(p.0, p.1 - 1)) {
        if hm[p.0][p.1 - 1] != 9 && hm[p.0][p.1 - 1] > curr {
            found.insert((p.0, p.1 - 1));
            basin_helper(hm, (p.0, p.1 - 1), found);
        }
    }

    if p.0 != (hm.len() - 1) && !found.contains(&(p.0 + 1, p.1)) {
        if hm[p.0 + 1][p.1] != 9 && hm[p.0 + 1][p.1] > curr {
            found.insert((p.0 + 1, p.1));
            basin_helper(hm, (p.0 + 1, p.1), found);
        }
    }
    if p.1 != (hm[0].len() - 1) && !found.contains(&(p.0, p.1 + 1)) {
        if hm[p.0][p.1 + 1] != 9 && hm[p.0][p.1 + 1] > curr {
            found.insert((p.0, p.1 + 1));
            basin_helper(hm, (p.0, p.1 + 1), found);
        }
    }
}

fn process_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|&b| b - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 15;
        let actual = day09a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 1134;
        let actual = day09b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
