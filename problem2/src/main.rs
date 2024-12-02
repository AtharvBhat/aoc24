use std::fs;

//part1
fn verify_report(report: Vec<i32>)->i32{
    let iter = report.windows(2);
    let mut increasing: bool = false;
    let mut decreasing: bool = false;

    for window in iter {
        let diff: i32 = window[1] - window[0];

        match diff.abs() {
            1..=3 => (),
            _ => return 0,
        }

        match diff {
            0.. => increasing = true,
            _ => decreasing = true,
        }

        if increasing && decreasing {
            return 0;
        }
    }
    1
}

fn verify_report_part1(report: &String) -> i32 {
    let report: Vec<i32> = report
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    verify_report(report)    
}

//part2
fn create_permutations(report: &String)-> Vec<Vec<i32>> {
    let report: Vec<i32> = report
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut permutations:Vec<Vec<i32>> = Vec::new();

    for (i, _) in report.iter().enumerate(){
        let mut temp:Vec<i32> = report.clone();
        temp.remove(i);
        permutations.push(temp);
    }
    permutations
}

fn verify_report_part2(report: &String) -> i32 {

    let permutations = create_permutations(report);
    for permutation in permutations{
        match verify_report(permutation){
            1 => return 1,
            _ => ()
        }
    }
    0
}

fn main() {
    let reports: Vec<String> = fs::read_to_string("input.txt")
        .expect("File not found")
        .lines()
        .map(|line| line.to_string())
        .collect();

    //part 1
    let result: i32 = reports
        .iter()
        .map(|report| verify_report_part1(report))
        .sum();

    println!("Part 1: {:?}", result);

    //part 1
    let result: i32 = reports
        .iter()
        .map(|report| verify_report_part2(report))
        .sum();

    println!("Part 2: {:?}", result);
}
