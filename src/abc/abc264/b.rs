
use proconio::input;

fn main() {

    input! {
        r: i32,
        c: i32,
    }

    let r = (r-8).abs();
    let c = (c-8).abs();

    if std::cmp::max(r, c) % 2 == 0 {
        println!("white");
    } else {
        println!("black");
    }

}