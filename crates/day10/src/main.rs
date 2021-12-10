const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day10a(INPUT));
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
}