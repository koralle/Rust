use fizzbuzz_tdd::fizzbuzz;

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(1), "1")
}

#[test]
fn test_fizzbuzz_case_fizz() {
    assert_eq!(fizzbuzz(3), "fizz")
}

#[test]
fn test_fizzbuzz_case_buzz() {
    assert_eq!(fizzbuzz(5), "buzz")
}

#[test]
fn test_fizzbuzz_case_fizzbuzz() {
    assert_eq!(fizzbuzz(15), "fizzbuzz")
}
