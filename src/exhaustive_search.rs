
// use std::io;
use proconio::input;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};

pub fn exhaustive_search() {

    // excercise for exhaustive search.
    
    println!("Input the number (0~255).");
    input! {
        v: u8,
    }

    // (note) if use std::io

    // let mut v = String::new();
    // io::stdin()
    //     .read_line(&mut v)
    //     .expect("Failed to read line.");
    // let v: u8 = v.trim()
    //     .parse()
    //     .expect("Please type a number!");

    // array length
    const N: u8 = 100;

    // initialize array with random numbers
    let mut arr = [0u8; N as usize];
    thread_rng().fill(&mut arr[..]);
    println!("random array: {:#?}", arr); // debug

    let start = Instant::now();

    // exhaustive search
    let mut exist: bool = false;
    let mut found_id: u8 = 0;

    for i in 0..N {
        if arr[i as usize]==v {
            exist = true;
            found_id = i;
            break
        }
    }

    let duration = start.elapsed();

    println!("existance: {}, id: {}", v, exist, found_id);
    println!("elapsed time: {:#?}", duration);

}