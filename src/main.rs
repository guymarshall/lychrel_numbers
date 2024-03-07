const LARGE_TEST: u128 = 89;
const VERY_LARGE_TEST: u128 = 1_186_060_307_891_929_990;
const POTENTIALLY_INFINITE: u128 = 196;

fn is_palindrome(number: u32) -> bool {
    number.to_string().chars().rev().collect::<String>() == number.to_string()
}

fn add_inverted(number: u32) -> u32 {
    let reversed_string: String = number.to_string().chars().rev().collect();
    number + reversed_string.parse::<u32>().unwrap_or(0)
}

fn main() {
    let original_number: u32 = 254;
    let mut number: u32 = original_number;
    let mut steps: u32 = 0;

    while !is_palindrome(number) {
        number = add_inverted(number);
        steps += 1;
    }

    println!("Number {} took {} steps", original_number, steps);
}
