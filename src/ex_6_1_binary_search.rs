use proconio::input;

pub fn exec() {

    println!("Binary search.");
    println!("input the length of your array:");
    input! {
        n: usize,
    }
    println!("input an integer sorted array (e.g. 1 2 5 10 21):");
    input! {
        a: [i32; n],
    }
    println!("input an integer you look for:");
    input! {
        k: i32,
    }

    let mut left = 0;
    let mut right = n - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid as usize]==k {
            println!("anwer: {}", mid);
            break
        } else if a[mid as usize] > k {
            right = mid - 1;
        } else if a[mid as usize] < k {
            left = mid + 1;
        }
    }

}