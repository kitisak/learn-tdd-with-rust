pub fn say(number: i8) -> String {
    let is_fizz = number%3 == 0;
    let is_buzz = number%5 == 0;
    let result = match (is_fizz, is_buzz) {
                    (true, true) => "fizzbuzz".to_string(),
                    (true, false) => "fizz".to_string(),
                    (false, true) => "buzz".to_string(),
                    (false, false) => number.to_string()
                };
    return result;
}
