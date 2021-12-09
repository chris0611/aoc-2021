const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day08a(INPUT));
    println!("{}", day08b(INPUT));
}

fn day08a(input: &str) -> usize {
    process_input(input).into_iter().fold(0, |cnt, (_, out)| {
        cnt + out
            .split_whitespace()
            .filter(|&d| match d.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    })
}

fn day08b(input: &str) -> usize {
    process_input(input).into_iter().fold(0, |res, (seg, out)| {
        let nums = seg.split_ascii_whitespace().collect::<Vec<_>>();

        let one = get_len(&nums, 2);
        let four = get_len(&nums, 4);

        res + decode(out, four, one)
    })
}

fn get_len<'a>(nums: &Vec<&'a str>, len: usize) -> &'a str {
    nums.into_iter().find(|&&n| n.len() == len).unwrap()
}

fn process_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .collect::<Vec<_>>()
}

fn decode(s: &str, four: &str, one: &str) -> usize {
    let disp = s.split_whitespace().collect::<Vec<_>>();

    disp.into_iter().enumerate().fold(0, |a, (i, d)| {
        a + decode_seg(d, &four, &one) * (10usize.pow(3 - i as u32))
    })
}

fn decode_seg(seg: &str, four: &str, one: &str) -> usize {
    match (seg.len(), in_common(seg, four), in_common(seg, one)) {
        (2, _, _) => 1,
        (3, _, _) => 7,
        (4, _, _) => 4,
        (7, _, _) => 8,
        (5, 2, _) => 2,
        (5, 3, 1) => 5,
        (5, 3, 2) => 3,
        (6, 4, _) => 9,
        (6, 3, 1) => 6,
        (6, 3, 2) => 0,
        (_, _, _) => panic!(),
    }
}

fn in_common(seg: &str, num: &str) -> usize {
    num.chars().fold(0, |a, c| a + seg.contains(c) as usize)
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
