// https://projecteuler.net/problem=17

fn get_digit_letter_length(num: u64) -> u64 {
    // println!("get_digit_letter_length: {}", num);
    match num {
        1 => 3, // One
        2 => 3, // Two
        3 => 5, // Three
        4 => 4, // Four
        5 => 4, // Five
        6 => 3, // Six
        7 => 5, // Seven
        8 => 5, // Eight
        9 => 4, // Nine
        _ => panic!("Input number out of range!"),
    }
}

fn get_tens_letter_length(num: u64) -> u64 {
    // println!("get_tens_letter_length: {}", num);
    match num {
        10 => 3, // Ten
        11 => 6, // Eleven
        12 => 6, // Twelve
        13 => 8, // Thirteen
        14 => 8, // Fourteen
        15 => 7, // Fifteen
        16 => 7, // Sixteen
        17 => 9, // Seventeen
        18 => 8, // Eighteen
        19 => 8, // Nineteen
        20 => 6, // Twenty
        30 => 6, // Thirty
        40 => 5, // Forty
        50 => 5, // Fifty
        60 => 5, // Sixty
        70 => 7, // Seventy
        80 => 6, // Eighty
        90 => 6, // Ninety
        _ => panic!("Input number out of range!"),
    }
}

fn number_letter_count(num: u64) -> u64 {
    match num {
        _ if num > 0 && num < 10 => get_digit_letter_length(num),
        _ if num >= 10 && num < 20 => get_tens_letter_length(num),
        _ if num >= 20 && num < 100 && num % 10 == 0 => get_tens_letter_length(num),
        _ if num >= 20 && num < 100 => {
            get_tens_letter_length(num - num % 10) + get_digit_letter_length(num % 10)
        }
        _ if num >= 100 && num < 1000 && num % 100 == 0 => get_digit_letter_length(num / 100) + 7,
        _ if num >= 100 && num < 1000 => {
            get_digit_letter_length(num / 100) + 7 + 3 + number_letter_count(num % 100)
        }
        _ if num == 1000 => 11,
        _ => panic!("Input number out of range!"),
    }
}

pub fn number_letter_counts() {
    let sum = (1..1001)
        .into_iter()
        .fold(0, |acc, num| acc + number_letter_count(num));
    println!("Letters used: {}", sum);
}
