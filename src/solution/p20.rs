// https://projecteuler.net/problem=20

use num_bigint::BigUint;

pub fn factorial_digit_sum(num: u32) {
    let sum: u32 = (2..num + 1)
        .into_iter()
        .fold(BigUint::from(1u32), |acc, element| {
            acc * BigUint::from(element)
        })
        .to_string()
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .sum();

    println!("Sum of the digits in the number {}!: {}", num, sum);
}
