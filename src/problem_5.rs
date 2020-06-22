use std::time;

pub fn run() {
    println!("first: {}", first());
    println!("first: {}ns", bench_first());
    println!("second: {}", second());
    println!("second: {}ns", bench_second());
}

pub fn gcd(a: i32, b: i32) -> i32 {
    if b > a {
        return gcd(b, a);
    }

    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

pub fn first() -> i32 {
    (1..20).fold(1, |acc, x| acc * x / gcd(acc, x))
}

pub fn bench_first() -> u128 {
    let mut total = 0;
    for _i in 0..10000 {
        let now = time::SystemTime::now();
        first();
        total += now.elapsed().expect("Error getting elapsed").as_nanos();
    }
    total / 10000
}

pub fn second() -> i32 {
    let mut res = 1;
    for i in 2..20 {
        res = res * i / gcd(i, res);
    }
    res
}

pub fn bench_second() -> u128 {
    let mut total = 0;
    for _i in 0..10000 {
        let now = time::SystemTime::now();
        second();
        total += now.elapsed().expect("Error getting elapsed").as_nanos();
    }
    total / 10000
}
