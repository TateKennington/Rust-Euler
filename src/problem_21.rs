pub fn run() {
    println!("first: {}", first());
    println!("second: {}", second());
    println!("third: {}", third());
}

fn factors(n: i32) -> i32 {
    let mut res = 0;
    for i in 1..n {
        if n % i == 0 {
            res += i;
        }
    }
    res
}

pub fn first() -> i32 {
    (2..10001)
        .map(|x| factors(x))
        .enumerate()
        .filter(|x| {
            let (a, b) = x;
            factors(*b) == (a + 2) as i32 && *b != (a + 2) as i32
        })
        .map(|x| x.1)
        .sum()
}

pub fn second() -> i32 {
    let sums: Vec<i32> = (2..10001).map(|x| factors(x)).collect();
    let mut res = 0;
    for (i, value) in sums.iter().enumerate() {
        let other = if *value > 10000 || *value < 2 {
            factors(*value)
        } else {
            sums[(*value - 2) as usize]
        };
        if (i + 2) as i32 == other && *value != (i + 2) as i32 {
            res += (i + 2) as i32;
        }
    }
    res
}

pub fn third() -> i32 {
    let mut sums: [i32; 10001] = [0; 10001];
    let mut res = 0;
    for i in 1..10001 {
        for j in (2 * i..10001).step_by(i) {
            sums[j] += i as i32;
        }
    }
    for i in 2..10001 {
        let value = if i > 10000 {
            factors(i)
        } else {
            sums[i as usize]
        };
        let other = if value > 10000 {
            factors(value)
        } else {
            sums[value as usize]
        };
        if i == other && i != value {
            res += i;
        }
    }
    res
}
