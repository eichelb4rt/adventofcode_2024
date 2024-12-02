mod task_1;
mod task_2;

fn main() {
    let test_result = task_1::main("test_input.txt");
    assert!(
        test_result == 2,
        "Test result failed: expected 2, got {test_result}."
    );
    println!("Solution task 1: {}", task_1::main("input.txt"));

    let test_result = task_2::main("test_input.txt");
    assert!(
        test_result == 4,
        "Test result failed: expected 4, got {test_result}."
    );
    println!("Solution task 1: {}", task_2::main("input.txt"));
}
