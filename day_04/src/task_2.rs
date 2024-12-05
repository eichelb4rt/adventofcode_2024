use crate::task_1::parse_input;

const XMAS_MASKS: [[[char; 3]; 3]; 4] = [
    [
        ['M', '.', 'M'], //
        ['.', 'A', '.'], //
        ['S', '.', 'S'], //
    ],
    [
        ['S', '.', 'M'], //
        ['.', 'A', '.'], //
        ['S', '.', 'M'], //
    ],
    [
        ['S', '.', 'S'], //
        ['.', 'A', '.'], //
        ['M', '.', 'M'], //
    ],
    [
        ['M', '.', 'S'], //
        ['.', 'A', '.'], //
        ['M', '.', 'S'], //
    ],
];

fn build_2d_windows(grid: &Vec<Vec<char>>, height: usize, width: usize) -> Vec<Vec<Vec<char>>> {
    let mut windows = Vec::new();
    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let window: Vec<Vec<char>> = grid[y..y + 3]
                .iter()
                .map(|line| line[x..x + 3].to_vec())
                .collect();
            windows.push(window);
        }
    }
    return windows;
}

fn matches_mask(window: &Vec<Vec<char>>, mask: [[char; 3]; 3]) -> bool {
    for y in 0..3 {
        for x in 0..3 {
            if mask[y][x] == '.' {
                continue;
            }
            if window[y][x] != mask[y][x] {
                return false;
            }
        }
    }
    return true;
}

fn is_xmas(window: &Vec<Vec<char>>) -> bool {
    for mask in XMAS_MASKS {
        if matches_mask(window, mask) {
            return true;
        }
    }
    return false;
}

fn solve_task(grid: Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let windows = build_2d_windows(&grid, height, width);
    return windows.iter().filter(|window| is_xmas(window)).count();
}

pub fn main(filename: &str) -> usize {
    let tasks = parse_input(filename);
    return solve_task(tasks);
}
