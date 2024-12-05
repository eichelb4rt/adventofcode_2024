use std::fs;

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
    (-1, -1),
];

pub fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let contents =
        fs::read_to_string(filename).expect(format!("Could not read file: {}", filename).as_str());
    return contents
        .lines()
        .map(|line| line.chars().into_iter().collect())
        .collect();
}

fn xmax_in_direction(
    grid: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    start_y: usize,
    start_x: usize,
    dy: i32,
    dx: i32,
) -> bool {
    let mut current_y = start_y as i32;
    let mut current_x = start_x as i32;
    for expected_char in ['M', 'A', 'S'] {
        current_y += dy;
        current_x += dx;
        if current_y < 0 || current_y >= height as i32 {
            return false;
        }
        if current_x < 0 || current_x >= width as i32 {
            return false;
        }
        if grid[current_y as usize][current_x as usize] != expected_char {
            return false;
        }
    }
    return true;
}

fn count_from_coordinate(
    grid: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    start_y: usize,
    start_x: usize,
) -> u32 {
    return DIRECTIONS
        .iter()
        .map(|(dy, dx)| xmax_in_direction(grid, height, width, start_y, start_x, *dy, *dx) as u32)
        .sum();
}

fn solve_task(grid: Vec<Vec<char>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut start_positions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'X' {
                start_positions.push((y, x));
            }
        }
    }
    return start_positions
        .iter()
        .map(|(y, x)| count_from_coordinate(&grid, height, width, *y, *x))
        .sum();
}

pub fn main(filename: &str) -> u32 {
    let tasks = parse_input(filename);
    return solve_task(tasks);
}
