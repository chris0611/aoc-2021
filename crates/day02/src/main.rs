const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("{}", day02a(INPUT));
    println!("{}", day02b(INPUT));
}

fn day02a(input: &'static str) -> usize {
    let mut h_pos = 0;
    let mut depth = 0;

    input
        .lines()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .map(|z| (z[0], z[1].parse::<u64>().unwrap()))
        .for_each(|(x, y)| match x {
            "forward" => h_pos += y,
            "down" => depth += y,
            "up" => depth -= y,
            _ => (),
        });

    h_pos as usize * depth as usize
}

fn day02b(input: &'static str) -> usize {
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .lines()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .map(|z| (z[0], z[1].parse::<u64>().unwrap()))
        .for_each(|(x, y)| match x {
            "forward" => {
                h_pos += y;
                depth += aim * y;
            }
            "down" => aim += y,
            "up" => aim -= y,
            _ => (),
        });

    h_pos as usize * depth as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 150;
        let actual = day02a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 900;
        let actual = day02b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
