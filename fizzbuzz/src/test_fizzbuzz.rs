#[test]
fn say_number() {
    assert_eq!(::fizzbuzz_library::say(1), "1");
    assert_eq!(::fizzbuzz_library::say(2), "2")
}

#[test]
fn say_fizz() {
    assert_eq!(::fizzbuzz_library::say(3), "fizz");
    assert_eq!(::fizzbuzz_library::say(6), "fizz")
}

#[test]
fn say_buzz() {
    assert_eq!(::fizzbuzz_library::say(5), "buzz");
    assert_eq!(::fizzbuzz_library::say(10), "buzz")
}

#[test]
fn say_fizzbuzz() {
    assert_eq!(::fizzbuzz_library::say(15), "fizzbuzz");
    assert_eq!(::fizzbuzz_library::say(30), "fizzbuzz")
}
