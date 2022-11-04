// https://projecteuler.net/problem=2

struct FibonacciSequence {
    current: u64,
    next: u64,
}

impl Iterator for FibonacciSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

pub fn even_fibonacci_numbers(num: u64) {
    let sum_of_even_values: u64 = FibonacciSequence {
        current: 1,
        next: 1,
    }
    .take_while(|&element| element < num)
    .map(|element| match element % 2 {
        0 => element,
        _ => 0,
    })
    .sum();

    println!(
        "Sum of even-valued terms under {}: {}",
        num, sum_of_even_values
    );
}
