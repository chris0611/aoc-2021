const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("{}", day01a(INPUT));
    println!("{}", day01b(INPUT));
}

fn day01a(input: &'static str) -> usize {
    input
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|&x| x[0] < x[1])
        .count()
}

fn day01b(input: &'static str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 7;
        let actual = day01a(TEST_INPUT);

        assert_eq!(expected, actual);

    }

    #[test]
    fn part_b() {
        let expected = 5;
        let actual = day01b(TEST_INPUT);

        assert_eq!(expected, actual);

    }
}
