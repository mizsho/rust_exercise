
use rand::Rng;

pub fn exec() {

    // Solve the Knapsack problem.
    // Pick items so that the total value is as large as possible
    // and the total weight is less than the limit.

    // N: number of items.
    // WMAX: weight limit.
    // arr_w: weight array.
    // arr_v: value array.

    const N: usize = 10;
    const WMAX: usize = 50;

    let mut arr_w = vec![0i32; N];
    let mut arr_v = vec![0i32; N];
    for i in 0..N {
        arr_w[i] = rand::thread_rng().gen_range(1..=100);
        arr_v[i] = rand::thread_rng().gen_range(1..=100);
    }
    println!("weight array: {:?}", arr_w);
    println!("value array: {:?}", arr_v);

    // dp[i][w]: the max of total value under
    //     - total weight <= w
    //     - pick first i items    
    let mut dp = vec![
        vec![0i32; WMAX+1];
        N+1
    ];

    for iu in 1..=N {
        for wu in 0..=WMAX {
            let w = wu as i32;
            // ith item is picked
            if w - arr_w[iu-1] > 0 {
                let b = dp[iu-1][(w - arr_w[iu-1]) as usize] + arr_v[iu-1];
                chmax(&mut dp[iu][wu], b);
            }
            // ith item is not picked
            let b = dp[iu-1][wu];
            chmax(&mut dp[iu][wu], b);            
        }
    }

    println!("maximized total value: {}", dp[N][WMAX]);
    println!("dp array: {:?}", dp);
}

fn chmax(a: &mut i32, b: i32) {

    // choose maximum value function for DP.
    // a: vector element (mutable reference)
    // b: reference number

    if *a < b {
        *a = b;
    }

}