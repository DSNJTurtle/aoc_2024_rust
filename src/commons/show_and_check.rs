pub fn show_and_check(part: &str, is_test: bool, result: i64, expected_result: Option<i64>) {
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
