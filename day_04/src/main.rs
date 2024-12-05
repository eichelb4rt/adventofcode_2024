mod task_1;
mod task_2;

fn main() {
    assert_eq!(task_1::main("test_input.txt"), 18);
    println!("Solution task 1: {}", task_1::main("input.txt"));

    assert_eq!(task_2::main("test_input.txt"), 9);
    println!("Solution task 2: {}", task_2::main("input.txt"));
}
