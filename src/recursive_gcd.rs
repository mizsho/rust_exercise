
use proconio::input;
use std::time::{Duration, Instant};

pub fn exec() {

    println!("Input two numbers m, n (0~65535).");
    input! {
        m: u16,
        n: u16,
    }

    let start = Instant::now();
    println!("gcd({}, {}) = {}", m, n, gcd(m, n))
    let duration = start.elapsed();
    println!("elapsed time: {:#?}", duration);

}

pub fn gcd(m: u16, n: u16) -> u16 {

    // recursively calc GCD by euclidean algorithm.
    // GCD(m, n) = GCD(n, r), r = m%n
    
    if n==0 {
        return m;
    } else {
        return gcd(n, m%n)
    }

}