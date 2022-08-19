
use proconio::input;
use std::time::{Duration, Instant};

pub fn exec() {

    println!("Input a number n (0~255).");
    input! {
        n: u8,
    }

    let start = Instant::now();
    println!("fib({}) = {}", n, fib_recursive(n));
    let duration = start.elapsed();
    println!("elapsed time: {:#?}", duration);

}

pub fn fib_recursive(n: u8) -> u32 {

    // recursively calc Fibonacci sequence.
    // fib(0) = 0
    // fib(1) = 1
    // fib(n) = fib(n-1) + fib(n-2)
    
    match n {
        0 => 0,
        1 => 1,
        n => fib_recursive(n-1) + fib_recursive(n-2),
    }

}