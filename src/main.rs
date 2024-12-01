mod day1;

use std::path::Path;
fn main() {
    let path_day1 = Path::new("./inputs/day1.txt");
    let res1 = day1::part1(&path_day1);
    let res2 = day1::part2(&path_day1);
    println!("{}, {}", res1, res2.unwrap());
}
