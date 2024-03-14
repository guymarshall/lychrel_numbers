const LARGE_TEST: u128 = 89;
const VERY_LARGE_TEST: u128 = 1_186_060_307_891_929_990;
const POTENTIALLY_INFINITE: u128 = 196;

use num::{BigInt, One};

fn is_palindrome(number: &BigInt) -> bool {
    let number_string: String = number.to_str_radix(10);
    let reversed_string: String = number_string.chars().rev().collect();
    number_string == reversed_string
}

fn add_inverted(number: &BigInt) -> BigInt {
    number.clone() + BigInt::parse_bytes(number.to_str_radix(10).chars().rev().collect::<String>().as_bytes(), 10).unwrap_or(BigInt::one())
}

fn main() {
    let original_number: BigInt = POTENTIALLY_INFINITE.into();
    let mut number: BigInt = original_number.clone();
    let mut steps: u32 = 0;

    while !is_palindrome(&number) {
        number = add_inverted(&number);
        steps += 1;
        
        if steps % 1000 == 0 {
            println!("Step: {}", steps);
        }
    }

    println!("Number {} took {} steps", original_number, steps);
}