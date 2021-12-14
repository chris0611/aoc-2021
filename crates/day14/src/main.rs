use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day14(INPUT, 10));
    println!("{}", day14(INPUT, 40));
}

fn day14(input: &str, iters: usize) -> usize {
    let (init, rules) = process_input(input);

    let mut counts = init
        .as_bytes()
        .windows(2)
        .map(|win| ((win[0] as char, win[1] as char), 1))
        .collect::<HashMap<(char, char), usize>>();

    for _ in 0..iters {
        counts = update_polymer(counts, &rules);
    }

    let (max, min) = get_max_and_min(counts);

    max - min
}

fn get_max_and_min(counts: HashMap<(char, char), usize>) -> (usize, usize) {
    let mut alphabet: [usize; 256] = [0; 256];

    for ((_, c1), cnt) in counts {
        alphabet[c1 as u8 as usize] += cnt;
    }

    alphabet.into_iter().fold((usize::MIN, usize::MAX), |(max, min), n| {
        (if n > max { n } else { max }, if (n != 0) && (n < min) { n } else { min })
    })
}

fn update_polymer(
    counts: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_counts = HashMap::new();

    for ((c0, c1), cnt) in counts {
        let c = *rules.get(&(c0, c1)).unwrap();

        let n1 = new_counts.entry((c0, c)).or_insert(0);
        *n1 += cnt;

        let n2 = new_counts.entry((c, c1)).or_insert(0);
        *n2 += cnt;
    }

    new_counts
}

fn process_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let (init, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(s, c)| {
            let mut s_itr = s.chars();
            (
                (s_itr.next().unwrap(), s_itr.next().unwrap()),
                c.chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();

    (init, rules)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 1588;
        let actual = day14(TEST_INPUT, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 2188189693529;
        let actual = day14(TEST_INPUT, 40);

        assert_eq!(expected, actual);
    }
}
