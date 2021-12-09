const INPUT: &str = include_str!("../input.txt");

const A: u8 = 0x01;
const B: u8 = 0x02;
const C: u8 = 0x04;
const E: u8 = 0x10;
const F: u8 = 0x20;

fn main() {
    println!("{}", day08a(INPUT));
    println!("{}", day08b(INPUT));
}

fn day08a(input: &str) -> i32 {
    process_input(input).into_iter().fold(0, |cnt, (_, out)| {
        cnt + out
            .split_whitespace()
            .filter(|&d| match d.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    }) as i32
}

fn day08b(input: &str) -> i32 {
    let parsed = process_input(input);

    let mut result = 0;

    for &disp in parsed.iter() {
        let notes = disp.0.split_ascii_whitespace().collect::<Vec<_>>();
        let digits = disp.1.split_ascii_whitespace().collect::<Vec<_>>();

        let output = decode(&notes, &digits);

        result += output;
    }

    result
}

fn process_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .collect::<Vec<_>>()
}

fn decode(notes: &Vec<&str>, output: &Vec<&str>) -> i32 {
    const LEN_FIVES: [u8; 3] = [2, 3, 5];
    const MIN_FIVES: [&str; 3] = ["ace", "acf", "abf"];
    const LEN_SIXES: [u8; 3] = [0, 6, 9];
    const MIN_SIXES: [&str; 3] = ["abcef", "abef", "abcf"];

    let mut mapping: [u8; 7] = [0x7F; 7];
    let mut freq = [0; 7];

    notes
        .iter()
        .for_each(|&s| s.bytes().for_each(|b| freq[(b - 97) as usize] += 1));

    freq.iter().enumerate().for_each(|(i, &n)| match n {
        4 => {
            solved_bit(&mut mapping, i, E);
        }
        6 => {
            solved_bit(&mut mapping, i, B);
        }
        9 => {
            solved_bit(&mut mapping, i, F);
        }
        _ => (),
    });

    let &one = notes.iter().find(|note| note.len() == 2).unwrap();
    let &seven = notes.iter().find(|note| note.len() == 3).unwrap();

    for c in one.chars() {
        if !char_solved(&mapping, c) {
            solved_bit(&mut mapping, c as usize - 97, C);
        }
    }

    for c in seven.chars() {
        if !char_solved(&mapping, c) {
            solved_bit(&mut mapping, c as usize - 97, A);
        }
    }

    let solved_segments = solved_rows(&mapping);

    let mut display = String::new();

    for &digit in output {
        match digit.len() {
            2 => display.push('1'),
            3 => display.push('7'),
            4 => display.push('4'),
            7 => display.push('8'),
            5 => {
                let mut solved_in = Vec::new();

                for c in digit.chars() {
                    for (to, from) in solved_segments.iter() {
                        if c == *to {
                            solved_in.push(*from);
                        }
                    }
                }

                solved_in.sort_unstable();
                let mut sorted_str = String::new();

                for c in solved_in {
                    sorted_str.push(c);
                }

                for (i, &min_s) in MIN_FIVES.iter().enumerate() {
                    if sorted_str.eq(min_s) {
                        display.push_str(&LEN_FIVES[i].to_string());
                    }
                }
            }
            6 => {
                let mut solved_in = Vec::new();

                for c in digit.chars() {
                    for (to, from) in solved_segments.iter() {
                        if c == *to {
                            solved_in.push(*from);
                        }
                    }
                }

                solved_in.sort_unstable();
                let mut sorted_str = String::new();

                for c in solved_in {
                    sorted_str.push(c);
                }

                for (i, &min_s) in MIN_SIXES.iter().enumerate() {
                    if sorted_str.eq(min_s) {
                        display.push_str(&LEN_SIXES[i].to_string());
                    }
                }
            }
            _ => (),
        }
    }

    display.parse::<i32>().unwrap()
}

// Helper functions
fn solved_rows(mapping: &[u8; 7]) -> Vec<(char, char)> {
    let mut chars = Vec::new();

    for (row, &b) in mapping.iter().enumerate() {
        if b.count_ones() == 1 {
            let mapped = match b {
                A => 'a',
                B => 'b',
                C => 'c',
                E => 'e',
                F => 'f',
                _ => '?',
            };

            chars.push(((row as u8 + 97) as char, mapped));
        }
    }

    chars
}

#[inline]
fn char_solved(mapping: &[u8; 7], c: char) -> bool {
    if mapping[c as usize - 97].count_ones() == 1 {
        true
    } else {
        false
    }
}

#[inline]
fn solved_bit(mapping: &mut [u8; 7], row: usize, bit: u8) {
    unset_col(mapping, bit);
    mapping[row] = bit;
}

#[inline]
fn unset_col(mapping: &mut [u8; 7], bit: u8) {
    mapping.iter_mut().for_each(|b| *b &= !bit);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");
    const SMALL_TEST_INPUT: &str = include_str!("../small_test.txt");

    #[test]
    fn part_a() {
        let expected = 26;
        let actual = day08a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b_small() {
        let expected = 5353;
        let actual = day08b(SMALL_TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b_long() {
        let expected = 61229;
        let actual = day08b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
