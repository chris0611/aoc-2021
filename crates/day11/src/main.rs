const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", day11a(INPUT));
    println!("{}", day11b(INPUT));
}

fn day11a(input: &str) -> usize {
    let mut flashes = 0;

    let mut octopuses: [[u8; 10]; 10] = [[0; 10]; 10];

    process_input(input, &mut octopuses);

    for _ in 0..100 {
        update_energy(&mut octopuses);
        flash(&mut octopuses);

        flashes += octopuses.into_iter().fold(0, |cnt, row| {
            cnt + row.into_iter().filter(|&b| b == 0).count()
        });
    }

    flashes
}

fn day11b(input: &str) -> usize {
    let mut result = 0;

    let mut octopuses: [[u8; 10]; 10] = [[0; 10]; 10];

    process_input(input, &mut octopuses);

    for day in 1.. {
        update_energy(&mut octopuses);
        flash(&mut octopuses);

        let flashes = octopuses.into_iter().fold(0, |cnt, row| {
            cnt + row.into_iter().filter(|&b| b == 0).count()
        });

        if flashes == 100 {
            result = day;
            break;
        }
    }

    result
}

fn process_input(input: &str, arr: &mut [[u8; 10]; 10]) {
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, b)| {
            arr[i][j] = b - b'0';
        })
    });
}

fn update_energy(arr: &mut [[u8; 10]; 10]) {
    for i in 0..10 {
        for j in 0..10 {
            arr[i][j] += 1;
        }
    }
}

fn flash(arr: &mut [[u8; 10]; 10]) {
    for i in 0..10 {
        for j in 0..10 {
            if arr[i][j] > 9 {
                update_cell(arr, i, j);
            }
        }
    }
}

fn update_cell(arr: &mut [[u8; 10]; 10], i: usize, j: usize) {
    if arr[i][j] != 0 {
        arr[i][j] += 1;

        if arr[i][j] > 9 {
            arr[i][j] = 0;

            if i != 0 {
                if j != 0 {
                    update_cell(arr, i - 1, j - 1);
                }
                if j != 9 {
                    update_cell(arr, i - 1, j + 1);
                }

                update_cell(arr, i - 1, j);
            }

            if i != 9 {
                if j != 0 {
                    update_cell(arr, i + 1, j - 1);
                }

                if j != 9 {
                    update_cell(arr, i + 1, j + 1);
                }
                update_cell(arr, i + 1, j);
            }

            if j != 0 {
                update_cell(arr, i, j - 1);
            }

            if j != 9 {
                update_cell(arr, i, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part_a() {
        let expected = 1656;
        let actual = day11a(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part_b() {
        let expected = 195;
        let actual = day11b(TEST_INPUT);

        assert_eq!(expected, actual);
    }
}
