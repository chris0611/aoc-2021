fn main() {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let result = input.windows(2).filter(|&x| x[0] < x[1]).count();

    println!("{}", result);
}
