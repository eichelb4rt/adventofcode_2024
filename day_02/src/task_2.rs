use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let contents =
        fs::read_to_string(filename).expect(format!("Failed to read file: {}", filename).as_str());
    let mut reports = Vec::new();
    for line in contents.lines() {
        reports.push(
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }
    return reports;
}

fn is_safe(report: &Vec<i32>) -> bool {
    for bad_index in 0..report.len() {
        let increasing = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_index)
            .map(|(_, e)| e)
            .collect::<Vec<&i32>>()
            .windows(2)
            .all(|w| w[0] < w[1]);
        let decreasing = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_index)
            .map(|(_, e)| e)
            .collect::<Vec<&i32>>()
            .windows(2)
            .all(|w| w[0] > w[1]);
        let difference_ok = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_index)
            .map(|(_, e)| e)
            .collect::<Vec<&i32>>()
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() <= 3);
        if (increasing || decreasing) && difference_ok {
            return true;
        }
    }

    return false;
}

fn solve_task(reports: Vec<Vec<i32>>) -> i32 {
    return reports
        .iter()
        .filter(|r| is_safe(r))
        .count()
        .try_into()
        .unwrap();
}

pub fn main(filename: &str) -> i32 {
    let reports = parse_input(filename);
    return solve_task(reports);
}
