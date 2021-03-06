const INPUT: &str = include_str!("../input.txt");
const DAYS_A: i32 = 80;
const DAYS_B: i32 = 256;

fn main() {
    println!("{}", solution(INPUT, DAYS_A));
    println!("{}", solution(INPUT, DAYS_B));
}

fn solution(input: &str, days: i32) -> u64 {
    let init_state: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut state_count: [u64; 9] = [0; 9];

    init_state
        .iter()
        .for_each(|&fish| state_count[fish as usize] += 1);

    for _ in 0..days {
        let inc = state_count[0];

        state_count.rotate_left(1);
        state_count[6] += inc;
    }

    state_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 5934;
        let actual = solution(TEST_INPUT, DAYS_A);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 26984457539;
        let actual = solution(TEST_INPUT, DAYS_B);

        assert_eq!(expected, actual);
    }
}
