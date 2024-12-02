mod task_1;

use task_1::task_1;

fn main() {
    let test_result = task_1("test_input.txt");
    assert!(
        test_result == 11,
        "Test result failed: expected 11, got {test_result}."
    );
    println!("Solution: {}", task_1("input.txt"));
}
