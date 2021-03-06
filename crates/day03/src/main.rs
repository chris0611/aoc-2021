const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day03a(INPUT));
    println!("{}", day03b(INPUT));
}

fn day03a(input: &str) -> u64 {
    let mut nums = 0;
    let mut counts: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let byte_iter = line.bytes().enumerate();

        if counts.is_empty() {
            byte_iter.for_each(|(_, b)| match b {
                b'1' => counts.push(1),
                _ => counts.push(0),
            });
        } else {
            byte_iter.for_each(|(i, b)| match b {
                b'1' => counts[i] += 1,
                _ => (),
            });
        }

        nums += 1;
    });

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    counts.iter().for_each(|&num| {
        if num > nums / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    });

    let gamma = binstr_to_num(&gamma);
    let epsilon = binstr_to_num(&epsilon);

    gamma * epsilon
}

fn day03b(input: &str) -> u64 {
    let nums: Vec<&str> = input.lines().collect();

    let oxy = helper_oxy(&nums, 0);
    let co2 = helper_co2(&nums, 0);

    oxy * co2
}

fn helper_oxy(input: &Vec<&str>, start: usize) -> u64 {
    if input.len() == 1 {
        return binstr_to_num(input[0]);
    }

    let mut nums: Vec<&str> = Vec::new();
    let mut ones: Vec<usize> = Vec::new();
    let mut zeros: Vec<usize> = Vec::new();

    for (i, &num) in input.iter().enumerate() {
        match num.as_bytes()[start] {
            b'1' => ones.push(i),
            b'0' => zeros.push(i),
            _ => (),
        }
    }

    if ones.len() >= zeros.len() {
        for index in ones {
            nums.push(input[index]);
        }
    } else {
        for index in zeros {
            nums.push(input[index]);
        }
    }

    helper_oxy(&nums, start + 1)
}

fn helper_co2(input: &Vec<&str>, start: usize) -> u64 {
    if input.len() == 1 {
        return binstr_to_num(input[0]);
    }

    let mut nums: Vec<&str> = Vec::new();
    let mut ones: Vec<usize> = Vec::new();
    let mut zeros: Vec<usize> = Vec::new();

    for (i, &num) in input.iter().enumerate() {
        match num.as_bytes()[start] {
            b'1' => ones.push(i),
            b'0' => zeros.push(i),
            _ => (),
        }
    }

    if zeros.len() <= ones.len() {
        for index in zeros {
            nums.push(input[index]);
        }
    } else {
        for index in ones {
            nums.push(input[index]);
        }
    }

    helper_co2(&nums, start + 1)
}

fn binstr_to_num(s: &str) -> u64 {
    s.bytes().enumerate().fold(0, |mut num, (i, b)| {
        match b {
            b'1' => num += 1 << (s.len() - 1 - i) as u64,
            _ => (),
        };
        num
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 198;
        let actual = day03a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 230;
        let actual = day03b(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_to_4() {
        let expected = 4;
        let actual = binstr_to_num("0100");

        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_to_123() {
        let expected = 123;
        let actual = binstr_to_num("01111011");

        assert_eq!(expected, actual);
    }
}
