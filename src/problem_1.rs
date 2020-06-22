use std::time;

pub fn run(){
    println!("first: {}", first());
    println!("first: {}ns", bench_first());
    println!("second: {}", second());
    println!("second: {}ns", bench_second());
}

pub fn first() -> i32{
    (0..1000).step_by(3).sum::<i32>() + (0..1000).step_by(5).sum::<i32>() - (0..1000).step_by(15).sum::<i32>()
}

pub fn second() -> i32{
    (1..1000).filter(|x| x%3==0 || x%5 == 0).sum()
}

pub fn bench_first() -> u128{
    let mut total = 0;
    for _i in 0..10000{
        let now = time::SystemTime::now();
        first();
        total += now.elapsed().expect("Error getting elapsed").as_nanos();
    }
    total / 10000
}

pub fn bench_second() -> u128{
    let mut total = 0;
    for _i in 0..10000{
        let now = time::SystemTime::now();
        second();
        total += now.elapsed().expect("Error getting elapsed").as_nanos();
    }
    total / 10000
}