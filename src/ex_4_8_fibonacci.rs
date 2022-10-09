
use proconio::input;
use std::time::Instant;

pub fn exec() {

    // calc fibonacci sequence.
    // fib(0) = 0
    // fib(1) = 1
    // fib(n) = fib(n-1) + fib(n-2)

    println!("Calc fibonacci sequence.");
    println!("Input a number n (0~50). Larger number will cause overflow.");
    input! {
        n: u8,
    }

    // (1) recursive version

    let start = Instant::now();

    println!("Recursive version");
    println!("fib({}) = {}", n, fib_recursive(n));

    let duration = start.elapsed();
    println!("elapsed time: {:#?}", duration);

    // (2) memo version

    let start = Instant::now();

    println!("memo version");
    println!("fib({}) = {}", n, fib_cache_exec(n));

    let duration = start.elapsed();
    println!("elapsed time: {:#?}", duration);

}

pub fn fib_recursive(n: u8) -> u64 {

    // recursively calc Fibonacci sequence.
    
    match n {
        0 => 0,
        1 => 1,
        n => fib_recursive(n-1) + fib_recursive(n-2),
    }

}

pub fn fib_cache_exec(n: u8) -> i64 {

    // recursively calc Fibonacci sequence.
    // use cache to reduce time.
    
    let n = n as usize;

    // initialize memo array as -1.
    let mut memo = [-1i64; 255];

    // pass mutable reference
    fib_cache(&mut memo, n)

}

fn fib_cache(memo: &mut [i64], n: usize) -> i64 {

    match n {
        0 => 0,
        1 => 1,
        n => {
            if memo[n] == -1 {
                // &mut is not needed because memo is already mutable reference here.
                memo[n] = fib_cache(memo, n-1) + fib_cache(memo, n-2);
            }
            memo[n]            
        }
    }

}