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

    let mut part1 = 0;
    let mut part2 = 0;

    for line in raw_updates.lines() {
        let update: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

        let (valid, middle) = check_update(&update, &rules);

        if valid {
            part1 += middle;
        } else {
            part2 += fix_update(&update, &rules);
        }
    }

    println!("Result Part 1: {}", part1);
    println!("Result Part 2: {}", part2);
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

fn check_update(pages: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> (bool, usize) {
    let empty_rules: Vec<usize> = vec![];

    if pages.iter().enumerate().skip(1).all(|(n, page)| {
        let page_rules = rules.get(page).unwrap_or(&empty_rules);
        pages
            .iter()
            .take(n)
            .all(|&prev_page| !page_rules.contains(&prev_page))
    }) {
        let middle = pages.iter().nth((pages.len() - 1) / 2).unwrap().clone();

        return (true, middle);
    }

    (false, 0)
}

fn fix_update(pages: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> usize {
    let mut reordered_pages: Vec<usize> = Vec::new();
    let empty_rules: Vec<usize> = Vec::new();

    pages.iter().for_each(|page| {
        let page_rules = rules.get(page).unwrap_or(&empty_rules);

        match reordered_pages
            .iter()
            .position(|prev_page| page_rules.contains(prev_page))
        {
            Some(index) => reordered_pages.insert(index, *page),
            None => reordered_pages.push(*page),
        }
    });

    let middle = reordered_pages
        .iter()
        .nth((pages.len() - 1) / 2)
        .unwrap()
        .clone();

    middle
}
