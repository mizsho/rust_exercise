
use std::cmp;
use rand::Rng;

const N: usize = 10;
const INF: i32 = 10000;

pub fn exec() {

    // solve frog problem.
    // arr: stage heights. 

    println!("===Solve frog problem by DP===");

    // initialize array with random numbers. (1~100)
    let mut arr = vec![0i32; N];
    for i in 0..N {
        arr[i] = rand::thread_rng().gen_range(1..=100);
    }
    println!("Stage heights: {:?}", arr);
    println!("Min cost: {}", frog_dp(&mut arr));

}

pub fn frog_dp(arr: &mut Vec<i32>) -> i32 {

    // solve frog problem by DP.
    // dp[i]: min cost for ith stage.
    
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
