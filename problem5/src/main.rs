use std::{collections::HashMap, fs};

fn split_rules_and_pages(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut rules: Vec<String> = vec![];
    let mut page_orders: Vec<String> = vec![];

    let mut empty = false;
    for line in lines {
        if line.is_empty() {
            empty = true;
            continue;
        }
        if empty {
            page_orders.push(line);
        } else {
            rules.push(line);
        }
    }
    (rules, page_orders)
}

fn parse_rules(rules: Vec<String>) -> HashMap<u32, Vec<u32>> {
    let mut rule_set = HashMap::new();

    for rule in rules {
        let mut rule = rule.split("|").map(|n| n.parse::<u32>().unwrap());
        let (a, b) = (rule.next().unwrap(), rule.next().unwrap());
        let entry = rule_set.entry(a).or_insert(vec![]);
        entry.push(b);
    }
    rule_set
}

fn verify_order(order: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut order = order.clone();
    while let Some(last) = order.pop() {
        if order.is_empty() {
            break;
        }
        if let Some(values) = rules.get(&last) {
            if order.iter().any(|x| values.contains(x)) {
                return false;
            }
        }
    }
    true
}

fn find_last(order: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> usize {
    let order = order.clone();
    let mut no_rule = 0;
    for (i, page) in order.iter().enumerate() {
        let mut compliment_order = order.clone();
        compliment_order.remove(i);
        if let Some(values) = rules.get(&page) {
            if compliment_order.iter().any(|x| values.contains(x)) {
                continue;
            }
            return i;
        } else {
            no_rule = i;
        }
    }
    no_rule
}

fn fix_order(order: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut fixed_order: Vec<u32> = vec![];
    let mut order = order.clone();
    loop {
        if order.is_empty() {
            break;
        }
        let last = order.remove(find_last(&order, rules));
        fixed_order.push(last);
    }
    fixed_order.reverse();
    fixed_order
}

fn main() {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .expect("File not found")
        .lines()
        .map(|l| l.to_string())
        .collect();

    let (rules, page_orders) = split_rules_and_pages(contents);
    let rules = parse_rules(rules);
    let page_orders: Vec<Vec<u32>> = page_orders
        .iter()
        .map(|l| l.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;
    for order in page_orders {
        //println!("Processing: {:?}", order);
        //part 1
        if verify_order(&order, &rules) {
            sum_1 += order[order.len() / 2]
        } else {
            let fixed_order = fix_order(&order, &rules);
            sum_2 += fixed_order[fixed_order.len() / 2]
        }
    }
    println!("Part 1: {sum_1}");
    println!("Part 2: {sum_2}")
}
