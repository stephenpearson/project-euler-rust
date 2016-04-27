// Problem 126

fn ring(x: i32, y: i32, l: i32) -> i32 {
    return 2 * x + 2 * y + 4 * (l - 1);
}

fn cap(x: i32, y: i32, l: i32) -> i32 {
    let mut total = x * y;
    for i in 1..l {
        total += ring(x, y, i);
    }
    return total;
}

fn layer(x: i32, y :i32, z: i32, l: i32) -> i32 {
    return 2 * cap(x, y, l) + z * ring(x, y, l);
}

fn show(count: &Vec<i32>) {
    for (i, &item) in count.iter().enumerate() {
        if item == 1000 {
            println!("Answer: {}", i);
            break;
        }
    }
}

pub fn run(_: Vec<String>) {
    let mut count = vec![0; 31000];
    let mut lim = 1;
    let mut x = 1;
    let mut y = 1;
    let mut z = 1;
    while lim < 10000 {
        for l in 1..30000 {
            let cubes = layer(x, y, z, l) as usize;
            if cubes > 30000 {
                break;
            }
            count[cubes] += 1;
        }
        if z < y {
            z += 1;
        } else if y < x {
            y += 1;
            z = 1;
        } else if x < lim {
            x += 1;
            y = 1;
            z = 1;
        } else {
            lim += 1;
            println!("lim = {}", lim);
            show(&count);
            x += 1;
            y = 1;
            z = 1;
        }
    }

    for (i, &item) in count.iter().enumerate() {
        if item == 1000 {
            println!("Answer: {}", i);
            break;
        }
    }
}
