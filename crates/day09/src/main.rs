const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day09a(INPUT));
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
}
