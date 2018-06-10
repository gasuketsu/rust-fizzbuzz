fn fizzbuzz(val: i32) -> String {
    match val {
        n if n % 3 == 0 && n % 5 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => val.to_string()
    }
}

fn main() {
    for i in 1..101 {
        println!("{}", fizzbuzz(i));
    }
}
