// Problem 3

use common::prime;

pub fn run(_: Vec<String>) {
    let value = 600851475143;
    let sqrt_value = (value as f64).sqrt() as i64;
    for i in (2..sqrt_value).rev() {
        if value % i == 0 && prime::is_prime(i) {
            println!("Answer: {}", i);
            break;
        }
    }
}
