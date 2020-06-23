use std::time;

pub fn run() {
    println!("first: {}", first());
    println!("first: {}ns", bench_first());
    println!("second: {}", second());
    println!("second: {}ns", bench_second());
}

pub fn first() -> i32 {
    let sum: i32 = 50 * 101;
    sum * sum - (1..101).map(|x| x * x).sum::<i32>()
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
    let sum: i32 = 50 * 101;
    (1..101).map(|x| x * (sum - x)).sum::<i32>()
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
