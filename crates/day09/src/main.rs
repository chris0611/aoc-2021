use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day09a(INPUT));
    println!("{}", day09b(INPUT));
}

fn day09a(input: &str) -> usize {
    let heightmap: Vec<Vec<u32>> = process_input(input);

    let mut result = 0;

    for (i, row) in heightmap.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == 9 {
                continue;
            }
            if i != 0 {
                if heightmap[i - 1][j] <= col {
                    continue;
                }
            }
            if j != 0 {
                if row[j - 1] <= col {
                    continue;
                }
            }
            if i != heightmap.len() - 1 {
                if heightmap[i + 1][j] <= col {
                    continue;
                }
            }
            if j != row.len() - 1 {
                if row[j + 1] <= col {
                    continue;
                }
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
        for (j, &col) in row.iter().enumerate() {
            if col == 9 {
                continue;
            }
            if i != 0 {
                if heightmap[i - 1][j] <= col {
                    continue;
                }
            }
            if j != 0 {
                if row[j - 1] <= col {
                    continue;
                }
            }
            if i != heightmap.len() - 1 {
                if heightmap[i + 1][j] <= col {
                    continue;
                }
            }
            if j != row.len() - 1 {
                if row[j + 1] <= col {
                    continue;
                }
            }
            basins.push(basin_finder(&heightmap, (i, j)));
        }
    }

    let mut max_three = [0; 3];

    basins.into_iter().for_each(|b| {
        let index = find_min(&max_three);
        if b > max_three[index] {
            max_three[index] = b;
        }
    });

    max_three.iter().product()
}

fn find_min(arr: &[usize; 3]) -> usize {
    let mut lowest = usize::MAX;
    let mut index = 0;

    for i in 0..3 {
        if arr[i] < lowest {
            lowest = arr[i];
            index = i;
        }
    }
    index
}

fn basin_finder(hm: &Vec<Vec<u32>>, init: (usize, usize)) -> usize {
    let mut found: HashSet<(usize, usize)> = HashSet::new();

    found.insert((init.0, init.1));
    helper(hm, init, &mut found);

    found.len()
}

fn helper(hm: &Vec<Vec<u32>>, p: (usize, usize), found: &mut HashSet<(usize, usize)>) {
    let curr = hm[p.0][p.1];

    if p.0 != 0 && !found.contains(&(p.0 - 1, p.1)) {
        if hm[p.0 - 1][p.1] != 9 && hm[p.0 - 1][p.1] > curr {
            found.insert((p.0 - 1, p.1));
            helper(hm, (p.0 - 1, p.1), found);
        }
    }

    if p.1 != 0 && !found.contains(&(p.0, p.1 - 1)) {
        if hm[p.0][p.1 - 1] != 9 && hm[p.0][p.1 - 1] > curr {
            found.insert((p.0, p.1 - 1));
            helper(hm, (p.0, p.1 - 1), found);
        }
    }

    if p.0 != (hm.len() - 1) && !found.contains(&(p.0 + 1, p.1)) {
        if hm[p.0 + 1][p.1] != 9 && hm[p.0 + 1][p.1] > curr {
            found.insert((p.0 + 1, p.1));
            helper(hm, (p.0 + 1, p.1), found);
        }
    }
    if p.1 != (hm[0].len() - 1) && !found.contains(&(p.0, p.1 + 1)) {
        if hm[p.0][p.1 + 1] != 9 && hm[p.0][p.1 + 1] > curr {
            found.insert((p.0, p.1 + 1));
            helper(hm, (p.0, p.1 + 1), found);
        }
    }
}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
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
