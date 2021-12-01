fn main() {
    let input = include_str!("../input.txt");

    let nums: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let res: Vec<i32> = nums.windows(3).map(|x| x.iter().sum()).collect();

    println!("{:?}", res.windows(2).filter(|x| x[0] < x[1]).count());
}
