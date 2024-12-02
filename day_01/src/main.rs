mod task_1;
mod task_2;

use task_1::task_1;
use task_2::task_2;

fn main() {
    let test_result = task_1("test_input.txt");
    assert!(
        test_result == 11,
        "Test result failed: expected 11, got {test_result}."
    );
    println!("Solution task 1: {}", task_1("input.txt"));

    let test_result = task_2("test_input.txt");
    assert!(
        test_result == 31,
        "Test result failed: expected 31, got {test_result}."
    );
    println!("Solution task 2: {}", task_2("input.txt"));
}
