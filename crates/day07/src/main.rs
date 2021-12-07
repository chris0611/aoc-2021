const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day07a(INPUT));
    println!("{}", day07b(INPUT));
}

fn day07a(input: &str) -> i32 {
    let mut nums = parse_input(input);

    nums.sort_unstable();

    let median = if nums.len() % 2 == 0 {
        (nums[nums.len() / 2] + nums[nums.len() / 2 - 1]) / 2
    } else {
        nums[nums.len() / 2]
    };

    nums.iter().map(|n| (n - median).abs()).sum()
}

fn day07b(input: &str) -> i32 {
    let nums = parse_input(input);

    let sum: i32 = nums.iter().sum();
    let mean = sum as f64 / nums.len() as f64;

    let mean_c = mean.ceil() as i32;
    let mean_f = mean.floor() as i32;

    let sum_c: i32 = nums
        .iter()
        .fold(0, |sum, &n| {
            let dist = (n - mean_c).abs();
            sum + (dist * (dist + 1) / 2)
        });

    let sum_f: i32 = nums
        .iter()
        .fold(0, |sum, &n| {
            let dist = (n - mean_f).abs();
            sum + (dist * (dist + 1) / 2)
        });

    sum_c.min(sum_f)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 37;
        let actual = day07a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 168;
        let actual = day07b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
