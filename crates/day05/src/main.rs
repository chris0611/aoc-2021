use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day05a(INPUT));
    println!("{}", day05b(INPUT));
}

fn day05a(input: &str) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let pairs = input
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    pairs.iter().for_each(|pair| {
        let (p1, p2) = pair.split_at(1);
        let (x1, y1) = p1[0].split_once(",").unwrap();
        let (x2, y2) = p2[0].split_once(",").unwrap();

        let x1 = x1.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();

        if x1 == x2 {
            for p in y1.min(y2)..=y2.max(y1) {
                if map.contains_key(&(x1, p)) {
                    let &last_val = map.get(&(x1, p)).unwrap();
                    map.insert((x1, p), last_val + 1);
                } else {
                    map.insert((x1, p), 1);
                }
            }
        } else if y1 == y2 {
            for p in x1.min(x2)..=x2.max(x1) {
                if map.contains_key(&(p, y1)) {
                    let &last_val = map.get(&(p, y1)).unwrap();
                    map.insert((p, y1), last_val + 1);
                } else {
                    map.insert((p, y1), 1);
                }
            }
        }
    });

    map.values().filter(|&v| *v > 1).count() as i32
}

fn day05b(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), u8> = HashMap::new();
    let mut count = 0;

    let pairs = input
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    pairs.iter().for_each(|pair| {
        let (p1, p2) = pair.split_at(1);
        let (x1, y1) = p1[0].split_once(",").unwrap();
        let (x2, y2) = p2[0].split_once(",").unwrap();

        let x1 = x1.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();

        if x1 == x2 {
            for p in y1.min(y2)..=y2.max(y1) {
                match map.get(&(x1, p)) {
                    Some(&n) => {
                        if n == 1 {
                            count += 1;
                        }
                        map.insert((x1, p), n + 1);
                    }
                    None => {
                        map.insert((x1, p), 1);
                    }
                }
            }
        } else if y1 == y2 {
            for p in x1.min(x2)..=x2.max(x1) {
                match map.get(&(p, y1)) {
                    Some(&n) => {
                        if n == 1 {
                            count += 1;
                        }
                        map.insert((p, y1), n + 1);
                    }
                    None => {
                        map.insert((p, y1), 1);
                    }
                }
            }
        // Diagonals
        } else {
            let dec_y;

            let x_s;
            let x_e;
            let mut y;

            if x1 < x2 {
                x_s = x1;
                x_e = x2;

                y = y1;

                dec_y = if y1 < y2 { false } else { true };
            } else {
                x_s = x2;
                x_e = x1;

                y = y2;

                dec_y = if y1 > y2 { false } else { true };
            }

            for x in x_s..=x_e {
                match map.get(&(x, y)) {
                    Some(&n) => {
                        if n == 1 {
                            count += 1;
                        }
                        map.insert((x, y), n + 1);
                    }
                    None => {
                        map.insert((x, y), 1);
                    }
                }

                if dec_y {
                    y -= 1;
                } else {
                    y += 1;
                }
            }
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 5;
        let actual = day05a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 12;
        let actual = day05b(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn all_diags() {
        const DIAG_TEST: &str = include_str!("../diag_test.txt");

        let expected = 2;
        let actual = day05b(DIAG_TEST);

        assert_eq!(expected, actual);
    }
}
