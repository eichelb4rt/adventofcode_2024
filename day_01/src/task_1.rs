use std::fs;

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

fn solve_task(mut locations_1: Vec<i32>, mut locations_2: Vec<i32>) -> i32 {
    locations_1.sort();
    locations_2.sort();
    let mut distance = 0;
    for (id_1, id_2) in locations_1.iter().zip(locations_2.iter()) {
        distance += (id_1 - id_2).abs();
    }
    return distance;
}

pub fn task_1(filename: &str) -> i32 {
    let (locations_1, locations_2) = parse_input(filename);
    return solve_task(locations_1, locations_2);
}
