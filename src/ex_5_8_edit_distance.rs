
use proconio::input;

pub fn exec() {

    // Calc the edit distance between two Strings s,t.
    // The following edit operations are allowed:
    // (1) change (a letter of string s)
    // (2) delete (a letter of string s)
    // (3) insert (a letter in string s)
    // "insert" is equivalent as "delete" a letter of string t.

    println!("input two strings:");
    input! {
        s: String,
        t: String,
    }

    // dp[i][j]: the edit distance of
    // first i letters of string s and first j letters of string t.
    const INF: usize = 10000;
    let mut dp = vec![
        vec![INF; t.len()+1];
        s.len()+1
    ];

    // initial condition
    dp[0][0] = 0;

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            // (1) change
            if (i > 0) && (j > 0) {
                // To pick ith char, use i-1. 
                if s.chars().nth(i-1) != t.chars().nth(j-1) {
                    let b = dp[i-1][j-1] + 1;
                    chmin(&mut dp[i][j], b);
                } else {
                    let b = dp[i-1][j-1];
                    chmin(&mut dp[i][j], b);
                }
            }

            // (2) delete a letter of s
            if i > 0 {
                let b = dp[i-1][j] + 1;
                chmin(&mut dp[i][j], b);
            }

            // (3) insert: delete a letter of t
            if j > 0 {
                let b = dp[i][j-1] + 1;
                chmin(&mut dp[i][j], b);
            }
        }
    }

    println!("edit distance: {}", dp[s.len()][t.len()]);
    println!("dp array: {:?}", dp);
}

fn chmin(a: &mut usize, b: usize) {

    // choose min value.
    // a: vector element (mutable reference)
    // b: reference number
    
    if *a > b {
        *a = b;
    }

}