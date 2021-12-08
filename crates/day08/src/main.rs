const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day08a(INPUT));
    println!("{}", day08b(INPUT));
}

fn day08a(input: &str) -> i32 {
    let notes = input
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .collect::<Vec<_>>();

    let output = notes
        .iter()
        .map(|&n| n.1.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    output.iter().fold(0, |cnt, v| {
        cnt + v
            .into_iter()
            .filter(|&&sig| match sig.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    }) as i32
}

fn day08b(input: &str) -> u64 {
    
    20
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 26;
        let actual = day08a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 61229;
        let actual = day08b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
