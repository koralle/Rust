pub fn fizzbuzz(num: u32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        return String::from("fizzbuzz");
    } else if num % 3 == 0 {
        return String::from("fizz");
    } else if num % 5 == 0 {
        return String::from("buzz");
    }

    return num.to_string();
}
