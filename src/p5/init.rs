// Problem 4

fn is_indivisible_upto(number: i64, upto: i64) -> bool {
    for i in 2..upto {
        if number % i != 0 {
            return false;
        }
    }
    return true;
}

pub fn run(_: Vec<String>) {
    let mut number = 3;
    while ! is_indivisible_upto(number, 20) {
        number += 1;
    }
    println!("Answer: {}", number);
}
