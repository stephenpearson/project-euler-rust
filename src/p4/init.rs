// Problem 4

fn is_palindrome(value: i32) -> bool {
    let mut reversed = 0;
    let mut acc = value.clone();
    while acc > 0 {
        let digit = acc % 10;
        acc = acc / 10;
        reversed = reversed * 10 + digit;
    }
    return reversed == value;
}

pub fn run(_: Vec<String>) {
    let mut largest = 0;
    for i in 0..999 {
        for j in 0..999 {
            let val = i * j;
            if is_palindrome(val) && val > largest {
                largest = val;
            }
        }
    }
    println!("Answer: {}", largest);
}
