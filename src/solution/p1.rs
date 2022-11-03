// https://projecteuler.net/problem=1

pub fn multiples_of_3_or_5(num: u32) {
    let sum = (0..num).fold(0, |acc, element| {
        if element % 3 == 0 || element % 5 == 0 {
            return acc + element;
        }
        acc
    });
    println!("Sum of all multiples of 3 or 5 under {}: {}", num, sum);
}
