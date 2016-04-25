// Problem 2

pub fn run(_: Vec<String>) {
    let mut tot = 0;
    let mut t1 = 1;
    let mut t2 = 2;
    while t2 < 4000000 {
        if t2 % 2 == 0 {
            tot += t2;
        }
        let t3 = t2;
        t2 = t1 + t2;
        t1 = t3;
    }
    println!("Answer: {}", tot);
}
