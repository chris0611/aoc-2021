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

    let mut population = init_state.len() as u64;

    for _ in 0..days {
        let mut carry = 0;

        for time in (0..=8).rev() {
            match time {
                0 => {
                    population += state_count[0];
                    state_count[8] += state_count[0];
                    state_count[6] += state_count[0];
                    state_count[0] = carry;
                }
                1..=8 => {
                    let tmp = state_count[time];
                    state_count[time] = carry;
                    carry = tmp;
                }
                _ => (),
            }
        }
    }

    population
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
