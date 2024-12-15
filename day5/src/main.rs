use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;

fn main() {
    println!("Advent of Code 2024 - Day 5");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let mut buffer_iter = buffer.split("\n\n");

    let raw_rules = buffer_iter.next().unwrap();
    let raw_updates = buffer_iter.next().unwrap();

    let rules = create_rulebook(raw_rules);

    let mut result = 0;

    for line in raw_updates.lines() {
        result += check_update(line, &rules);
    }

    println!("Result: {}", result);
}

fn create_rulebook(raw_rules: &str) -> HashMap<usize, Vec<usize>> {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    raw_rules.lines().for_each(|line| {
        let mut pages_iter = line.split('|');
        let first = pages_iter.next().unwrap().parse::<usize>().unwrap();
        let second = pages_iter.next().unwrap().parse::<usize>().unwrap();

        let page_rule = rules.entry(first).or_insert(Vec::new());
        page_rule.push(second);
    });

    rules
}

fn check_update(raw_update: &str, rules: &HashMap<usize, Vec<usize>>) -> usize {
    let pages: Vec<usize> = raw_update.split(',').map(|x| x.parse().unwrap()).collect();
    let empty_rules: Vec<usize> = vec![];

    if pages.iter().enumerate().skip(1).all(|(n, page)| {
        let page_rules = rules.get(page).unwrap_or(&empty_rules);
        pages
            .iter()
            .take(n)
            .all(|&prev_page| !page_rules.contains(&prev_page))
    }) {
        return pages.iter().nth((pages.len() - 1) / 2).unwrap().clone();
    }

    0
}
