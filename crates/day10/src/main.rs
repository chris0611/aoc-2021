use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day10a(INPUT));
    println!("{}", day10b(INPUT));
}

fn day10a(input: &str) -> usize {
    let mut chunks: Vec<char> = Vec::new();
    let mut scores: [usize; 4] = [0, 0, 0, 0];

    input.lines().for_each(|line| {
        for c in line.chars() {
            match c {
                ')' => {
                    if chunks.pop().unwrap() != '(' {
                        scores[0] += 3;
                    }
                }
                ']' => {
                    if chunks.pop().unwrap() != '[' {
                        scores[1] += 57;
                    }
                }
                '}' => {
                    if chunks.pop().unwrap() != '{' {
                        scores[2] += 1197;
                    }
                }
                '>' => {
                    if chunks.pop().unwrap() != '<' {
                        scores[3] += 25137;
                    }
                }
                b => chunks.push(b),
            }
        }
        chunks.clear();
    });

    scores.iter().sum()
}

fn day10b(input: &str) -> usize {
    let mut chunks: Vec<char> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();

    let matching = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let scoring = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    input.lines().for_each(|line| {
        let mut corrupt = false;

        for c in line.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    if chunks.pop().unwrap() != *matching.get(&c).unwrap() {
                        corrupt = true;
                        break;
                    }
                }
                b => chunks.push(b),
            }
        }

        if !corrupt {
            let mut score = 0;

            while !chunks.is_empty() {
                let b = chunks.pop().unwrap();
                match b {
                    '(' | '[' | '{' | '<' => {
                        score *= 5;
                        score += scoring.get(&b).unwrap();
                    }
                    _ => (),
                }
            }
            scores.push(score);
        }

        chunks.clear();
    });

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 26397;
        let actual = day10a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 288957;
        let actual = day10b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
