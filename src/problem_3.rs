use std::time;

pub fn run() {
    println!("first: {}", first());
    println!("first: {}ns", bench_first());
    println!("second: {}", second());
    println!("second: {}ns", bench_second());
}

pub fn first() -> i64 {
    let mut n = 600851475143;
    let mut x = 2;
    while n != x {
        while n != x && n % x == 0 {
            n = n / x;
        }
        x += 1;
    }
    x
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

pub fn second() -> i64 {
    let mut n = 600851475143;
    let mut x = 3;
    while n % 2 == 0 {
        n = n / 2;
    }
    if n == 1 {
        return 2;
    }
    while n != x {
        while n != x && n % x == 0 {
            n = n / x;
        }
        x += 2;
    }
    x
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
