use std::{collections::HashMap, fs, iter::zip};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());

        let (a, b) = (nums.next().unwrap(), nums.next().unwrap());

        first.push(a);
        second.push(b);
    }

    first.sort();
    second.sort();

    // part 1

    let sum: i32 = zip(&first, &second).map(|(a, b)| (a - b).abs()).sum();

    println!("Solution Part 1 : {sum}");

    // Part 2

    let mut second_count = HashMap::new();

    for num in second {
        let count = second_count.entry(num).or_insert(0);
        *count += 1;
    }

    let sum: i32 = first
        .iter()
        .map(|num| num * second_count.get(num).unwrap_or(&0))
        .sum();

    println!("Solution Part 2 : {sum}");
}
