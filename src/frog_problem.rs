
use std::cmp;
use rand::Rng;

const N: usize = 10;
const INF: i32 = 10000;

pub fn exec() {

    // solve frog problem.
    // arr: height array for stages. 

    // initialize array with random numbers. (1~100)
    let mut arr = vec![0i32; N];
    for i in 0..N {
        arr[i] = rand::thread_rng().gen_range(1..=100);
    }
    println!("random array: {:?}", arr);

    // println!("{}", frog_dp(&mut arr));
    println!("{}", frog_dp_chmin(&mut arr));

}

pub fn frog_dp(arr: &mut Vec<i32>) -> i32 {

    // solve frog problem by DP.
    
    let mut dp = vec![INF; N];
    
    dp[0] = 0;
    dp[1] = (arr[1] - arr[0]).abs();

    for i in 2..N {
        dp[i] = cmp::min(
            dp[i-2] + (arr[i] - arr[i-2]).abs(),
            dp[i-1] + (arr[i] - arr[i-1]).abs(),
        )
    }

    dp[N-1]

}

pub fn frog_dp_chmin(arr: &mut Vec<i32>) -> i32 {

    // solve frog problem by DP.
    // use chmin function to think of "relaxation".
    
    let mut dp = vec![INF; N];
    
    dp[0] = 0;
    dp[1] = (arr[1] - arr[0]).abs();

    for i in 2..N {
        for j in 1..=2 {
            let b = dp[i-j] + (arr[i] - arr[i-j]).abs();
            chmin(&mut dp[i], b);
        }
    }

    dp[N-1]

}

fn chmin(a: &mut i32, b: i32) {

    // choose minimum value function for DP.
    // a: vector element (mutable reference)
    // b: reference number

    if *a > b {
        *a = b;
    }

}