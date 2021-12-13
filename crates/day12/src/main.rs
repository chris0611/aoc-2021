use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day12a(INPUT));
    println!("{}", day12b(INPUT));
}

fn day12a(input: &str) -> usize {
    let list = build_graph(input);

    search_caves(&list, false)
}

fn day12b(input: &str) -> usize {
    let list = build_graph(input);

    search_caves(&list, true)
}

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut list = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .for_each(|(e1, e2)| {
            if !list.contains_key(e1) {
                let mut tmp = Vec::new();
                tmp.push(e2);
                list.insert(e1, tmp);
            } else {
                let adj = list.get_mut(e1).unwrap();
                adj.push(e2);
            }

            if !list.contains_key(e2) {
                let mut tmp = Vec::new();
                tmp.push(e1);
                list.insert(e2, tmp);
            } else {
                let adj = list.get_mut(e2).unwrap();
                adj.push(e1);
            }
        });

    list
}

fn search_caves(list: &HashMap<&str, Vec<&str>>, part_b: bool) -> usize {
    let mut count = 0;

    helper(&mut count, "start", list, &HashSet::new(), !part_b);

    count
}

fn helper(
    cnt: &mut usize,
    curr: &str,
    list: &HashMap<&str, Vec<&str>>,
    visited: &HashSet<&str>,
    twice: bool,
) {
    if curr == "end" {
        *cnt += 1;
        return;
    }

    let mut cpy = visited.clone();

    if curr.chars().next().unwrap().is_ascii_lowercase() {
        cpy.insert(curr);
    }

    for cave in list.get(curr).unwrap() {
        if *cave == "start" {
            continue;
        }

        if !cpy.contains(cave) {
            helper(cnt, cave, list, &cpy, twice);
        } else {
            if !twice {
                helper(cnt, cave, list, &cpy, true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_SMALL: &str = include_str!("../test_small.txt");
    const TEST_INPUT_MEDIUM: &str = include_str!("../test_medium.txt");
    const TEST_INPUT_BIG: &str = include_str!("../test_big.txt");

    #[test]
    fn part_a_small() {
        let expected = 10;
        let actual = day12a(TEST_INPUT_SMALL);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_a_medium() {
        let expected = 19;
        let actual = day12a(TEST_INPUT_MEDIUM);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_a_big() {
        let expected = 226;
        let actual = day12a(TEST_INPUT_BIG);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b_small() {
        let expected = 36;
        let actual = day12b(TEST_INPUT_SMALL);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b_medium() {
        let expected = 103;
        let actual = day12b(TEST_INPUT_MEDIUM);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b_big() {
        let expected = 3509;
        let actual = day12b(TEST_INPUT_BIG);

        assert_eq!(expected, actual);
    }
}
