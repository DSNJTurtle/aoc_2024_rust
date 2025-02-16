use std::fmt::Debug;

pub fn show_and_check<T: std::fmt::Display + PartialEq + Debug>(
    part: &str,
    is_test: bool,
    result: T,
    expected_result: Option<T>,
) {
    println!("Part {}", part);
    if is_test {
        println!("For test file")
    } else {
        println!("For input file");
    }
    println!("Result: {}", result);
    println!();
    match expected_result {
        Some(e) => assert_eq!(result, e),
        _ => {}
    }
}
