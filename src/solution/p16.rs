// https://projecteuler.net/problem=16

use num_bigint::BigUint;

pub fn power_digit_sum(num: u32) {
    let sum: u32 = BigUint::from(2u32)
        .pow(1000)
        .to_string()
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .sum();

    println!("Sum of the digits of the number 2^{}: {}", num, sum);
}
