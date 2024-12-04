mod task_1;
mod task_2;

fn main() {
    let test_result = task_1::main("test_input_1.txt");
    assert!(
        test_result == 161,
        "Test result failed: expected 161, got {test_result}."
    );
    println!("Solution task 1: {}", task_1::main("input.txt"));

    let test_result = task_2::main("test_input_2.txt");
    assert!(
        test_result == 48,
        "Test result failed: expected 48, got {test_result}."
    );
    println!("Solution task 1: {}", task_2::main("input.txt"));
}
