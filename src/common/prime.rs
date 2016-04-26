// Prime number related helper functions

#[allow(unused_imports)]
use std::cmp::Ordering;
use std::iter::repeat;

pub fn is_prime(value: i64) -> bool {
    for i in 2..(value / 2) {
        if value % i == 0 {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
pub fn sieve(max: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = repeat(true).take(max).collect();
    sieve[0] = false;
    sieve[2] = false;
    for i in 2..(max / 2) {
        let mut j = i * 2;
        while j < max {
            sieve[j] = false;
            j += i;
        }
    }
    let result: Vec<usize> = sieve.iter().enumerate()
                                  .filter(|&(_, &val)| val)
                                  .map(|(i, &_)| i).collect();
    return result;
}

#[allow(dead_code)]
pub fn gcd(x: i64, y: i64) -> i64 {
    match x.cmp(&y) {
        Ordering::Equal => return x,
        Ordering::Less => return gcd(x, y - x),
        Ordering::Greater => return gcd(x - y, y)
    }
}
