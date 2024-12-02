use std::fs;
use counter::Counter;

fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let read_result = fs::read_to_string(filename);
    let contents = read_result.expect("Could not read file.");
    let mut locations_1 = Vec::new();
    let mut locations_2 = Vec::new();
    for line in contents.lines() {
        let entries: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        locations_1.push(entries[0]);
        locations_2.push(entries[1]);
    }
    return (locations_1, locations_2);
}

fn solve_task(locations_1: Vec<i32>, locations_2: Vec<i32>) -> i32 {
    let n_occurences = locations_2.iter().collect::<Counter<_>>();
    let mut similarity = 0;
    for id_1 in locations_1.iter() {
        similarity += id_1 * n_occurences[id_1] as i32;
    }
    return similarity;
}

pub fn task_2(filename: &str) -> i32 {
    let (locations_1, locations_2) = parse_input(filename);
    return solve_task(locations_1, locations_2);
}
