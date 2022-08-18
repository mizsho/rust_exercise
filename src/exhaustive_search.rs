
use std::io;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};

pub fn exhaustive_search() {

    // 乱数配列の中から指定した数値が何番目に存在するか判定.
    // 線形検索を用いる.
    
    // 検索する値
    println!("Input the number (0~255).");
    let mut v = String::new();
    io::stdin()
        .read_line(&mut v)
        .expect("Failed to read line.");
    let v: u8 = v.trim()
        .parse()
        .expect("Please type a number!");

    // 乱数配列の長さ
    const N: u8 = 100;

    // arrayを乱数で初期化
    let mut arr = [0u8; N as usize];
    thread_rng().fill(&mut arr[..]);
    println!("random array: {:#?}", arr); // debug

    let start = Instant::now();

    // 線形探索
    let mut exist: bool = false; // 存在判定
    let mut found_id: u8 = 0; // 何番目に存在するか

    for i in 0..N {
        if arr[i as usize]==v {
            exist = true;
            found_id = i;
            break
        }
    }

    let duration = start.elapsed();

    println!("指定した値{}の検索結果: {}, {}番目", v, exist, found_id);
    println!("経過時間: {:#?}", duration);

}