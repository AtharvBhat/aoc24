use regex::Regex;
use std::fs;

fn parse_mul(mul: String) -> i32 {
    let nums: Vec<i32> = mul
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    nums[0] * nums[1]
}

fn filter_commands(cmds: Vec<&str>) -> Vec<&str> {
    let mut valid_cmds: Vec<&str> = Vec::new();
    let mut cmd_toggle: bool = true;

    for cmd in cmds {
        if cmd.starts_with("mul") && cmd_toggle {
            valid_cmds.push(cmd);
        }

        if cmd.starts_with("do(") {
            cmd_toggle = true;
        }

        if cmd.starts_with("don't(") {
            cmd_toggle = false;
        }
    }
    valid_cmds
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("file not found")
        .trim()
        .to_owned();

    //part 1
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&contents).map(|m| m.as_str()).collect();

    let sum: i32 = matches
        .iter()
        .map(|m| parse_mul(m.to_owned().to_string()))
        .sum();

    println!("Part 1: {sum}");

    //part 2
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&contents).map(|m| m.as_str()).collect();

    let sum: i32 = filter_commands(matches)
        .iter()
        .map(|m| parse_mul(m.to_owned().to_string()))
        .sum();

    println!("Part 2: {sum}")
}
