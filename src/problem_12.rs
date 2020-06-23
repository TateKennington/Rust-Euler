use std::time;

pub fn run() {
    println!("first: {}", first());
    println!("first: {}ns", bench_first());
}

fn factors(n: i32) -> i32 {
    let mut res = 1;
    let mut curr = 2;
    let mut total = n;
    while total > 1 {
        let mut temp = 0;
        while total % curr == 0 {
            total /= curr;
            temp += 1;
        }
        res = res * (temp + 1);
        curr += 1;
    }
    res
}

pub fn first() -> i32 {
    (1..)
        .map(|x| x * (x + 1) / 2)
        .skip_while(|x| factors(*x) < 500)
        .next()
        .expect("value was none")
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
