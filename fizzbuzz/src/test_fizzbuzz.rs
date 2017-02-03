use fizzbuzz_library::say;

#[test]
fn say_number() {
    assert_eq!(say(1), "1");
    assert_eq!(say(2), "2")
}

#[test]
fn say_fizz() {
    assert_eq!(say(3), "fizz");
    assert_eq!(say(6), "fizz")
}

#[test]
fn say_buzz() {
    assert_eq!(say(5), "buzz");
    assert_eq!(say(10), "buzz")
}

#[test]
fn say_fizzbuzz() {
    assert_eq!(say(15), "fizzbuzz");
    assert_eq!(say(30), "fizzbuzz")
}
