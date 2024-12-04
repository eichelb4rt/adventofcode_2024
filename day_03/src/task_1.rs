use std::fs;

const MAX_NUM_SIZE: usize = 3;

enum MulState {
    Start,
    CharM,
    CharU,
    CharL,
    ParOpen,
    Num1(u32),
    Comma(u32),
    Num2(u32, u32),
    Accept(u32, u32),
}

fn state_machine_step(state: MulState, c: char) -> MulState {
    match state {
        MulState::Start => {
            if c == 'm' {
                return MulState::CharM;
            }
            return MulState::Start;
        }
        MulState::CharM => {
            if c == 'u' {
                return MulState::CharU;
            }
            return MulState::Start;
        }
        MulState::CharU => {
            if c == 'l' {
                return MulState::CharL;
            }
            return MulState::Start;
        }
        MulState::CharL => {
            if c == '(' {
                return MulState::ParOpen;
            }
            return MulState::Start;
        }
        MulState::ParOpen => {
            if c.is_digit(10) {
                return MulState::Num1(c.to_digit(10).unwrap());
            }
            return MulState::Start;
        }
        MulState::Num1(num) => {
            if c == ',' {
                return MulState::Comma(num);
            } else if c.is_digit(10) {
                if num.to_string().len() >= MAX_NUM_SIZE {
                    return MulState::Start;
                } else {
                    return MulState::Num1(num * 10 + c.to_digit(10).unwrap());
                }
            }
            return MulState::Start;
        }
        MulState::Comma(num_1) => {
            if c.is_digit(10) {
                return MulState::Num2(num_1, c.to_digit(10).unwrap());
            }
            return MulState::Start;
        }
        MulState::Num2(num_1, num_2) => {
            if c == ')' {
                return MulState::Accept(num_1, num_2);
            } else if c.is_digit(10) {
                if num_2.to_string().len() >= MAX_NUM_SIZE {
                    return MulState::Start;
                } else {
                    return MulState::Num2(num_1, num_2 * 10 + c.to_digit(10).unwrap());
                }
            }
            return MulState::Start;
        }
        _ => return MulState::Start,
    }
}

fn parse_input(filename: &str) -> Vec<(u32, u32)> {
    let contents =
        fs::read_to_string(filename).expect(format!("Failed to read file: {}", filename).as_str());
    let mut tasks = Vec::new();
    let mut state = MulState::Start;
    for c in contents.chars() {
        state = state_machine_step(state, c);
        if let MulState::Accept(num_1, num_2) = state {
            tasks.push((num_1, num_2));
            state = MulState::Start;
        }
    }
    return tasks;
}

fn solve_task(tasks: Vec<(u32, u32)>) -> u32 {
    return tasks
        .iter()
        .map(|(num_1, num_2)| num_1 * num_2)
        .sum::<u32>();
}

pub fn main(filename: &str) -> u32 {
    let tasks = parse_input(filename);
    return solve_task(tasks);
}
