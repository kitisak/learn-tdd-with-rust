pub fn say(number: i8) -> String {
    if number%15 == 0 {
        return "fizzbuzz".to_string()
    } else if number%3 == 0 {
        return "fizz".to_string()
    } else if number%5 == 0 {
        return "buzz".to_string()
    }
    number.to_string()
}
