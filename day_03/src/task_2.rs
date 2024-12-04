use std::fs;

const MAX_NUM_SIZE: usize = 3;

enum State {
    Start,
    MulCharM,
    MulCharU,
    MulCharL,
    MulParOpen,
    MulNum1(u32),
    MulComma(u32),
    MulNum2(u32, u32),
    MulAccept(u32, u32),
    EnableD,
    EnableO,
    EnableN,
    EnableApostrophe,
    EnableT,
    EnableParOpen(bool),
    EnableAccept(bool),
}

fn state_machine_step(state: State, c: char) -> State {
    match state {
        // ============================================================================
        // ================================ Mul States ================================
        // ============================================================================
        State::Start => {
            if c == 'm' {
                return State::MulCharM;
            }
            if c == 'd' {
                return State::EnableD;
            }
            return State::Start;
        }
        State::MulCharM => {
            if c == 'u' {
                return State::MulCharU;
            }
            return State::Start;
        }
        State::MulCharU => {
            if c == 'l' {
                return State::MulCharL;
            }
            return State::Start;
        }
        State::MulCharL => {
            if c == '(' {
                return State::MulParOpen;
            }
            return State::Start;
        }
        State::MulParOpen => {
            if c.is_digit(10) {
                return State::MulNum1(c.to_digit(10).unwrap());
            }
            return State::Start;
        }
        State::MulNum1(num) => {
            if c == ',' {
                return State::MulComma(num);
            } else if c.is_digit(10) {
                if num.to_string().len() >= MAX_NUM_SIZE {
                    return State::Start;
                } else {
                    return State::MulNum1(num * 10 + c.to_digit(10).unwrap());
                }
            }
            return State::Start;
        }
        State::MulComma(num_1) => {
            if c.is_digit(10) {
                return State::MulNum2(num_1, c.to_digit(10).unwrap());
            }
            return State::Start;
        }
        State::MulNum2(num_1, num_2) => {
            if c == ')' {
                return State::MulAccept(num_1, num_2);
            } else if c.is_digit(10) {
                if num_2.to_string().len() >= MAX_NUM_SIZE {
                    return State::Start;
                } else {
                    return State::MulNum2(num_1, num_2 * 10 + c.to_digit(10).unwrap());
                }
            }
            return State::Start;
        }
        // ===============================================================================
        // ================================ Enable States ================================
        // ===============================================================================
        State::EnableD => {
            if c == 'o' {
                return State::EnableO;
            }
            return State::Start;
        }
        State::EnableO => {
            if c == '(' {
                return State::EnableParOpen(true);
            }
            if c == 'n' {
                return State::EnableN;
            }
            return State::Start;
        }
        State::EnableN => {
            if c == '\'' {
                return State::EnableApostrophe;
            }
            return State::Start;
        }
        State::EnableApostrophe => {
            if c == 't' {
                return State::EnableT;
            }
            return State::Start;
        }
        State::EnableT => {
            if c == '(' {
                return State::EnableParOpen(false);
            }
            return State::Start;
        }
        State::EnableParOpen(enabled) => {
            if c == ')' {
                return State::EnableAccept(enabled);
            }
            return State::Start;
        }
        _ => return State::Start,
    }
}

fn parse_input(filename: &str) -> Vec<(u32, u32)> {
    let contents =
        fs::read_to_string(filename).expect(format!("Failed to read file: {}", filename).as_str());
    let mut tasks = Vec::new();
    let mut state = State::Start;
    let mut enabled = true;
    for c in contents.chars() {
        state = state_machine_step(state, c);
        if let State::EnableAccept(state_enabled) = state {
            enabled = state_enabled;
            state = State::Start;
        }
        if let State::MulAccept(num_1, num_2) = state {
            if enabled {
                tasks.push((num_1, num_2));
            }
            state = State::Start;
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
