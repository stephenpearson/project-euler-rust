// Problem 1

pub fn run(_: Vec<String>) {
    let mut tot = 0;
    for i in 1..1000 {
        if ( i % 3 == 0 ) || ( i % 5 == 0 ) {
            tot += i;
        }
    }
    println!("Answer: {}", tot);
}
