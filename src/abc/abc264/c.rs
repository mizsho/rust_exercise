
use proconio::input;

fn main() {

    input! {
        h1: usize,
        w1: usize,
        a: [[u32; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[u32; w2]; h2],
    }

    // ビット全探索
    // 1 << x = 2^x となり, iとjはすべての行と列の組をわたる.
    // <<は, 左bit shiftを表す.

    let mut ans = "No";

    for i in 0..(1 << h1) {
        for j in 0..(1 << w1) {

            // println!("i:{}, j:{}", i, j);

            // 行列aからi,jで指定した行,列を残した行列
            let mut vec_all = vec![];
            for i2 in 0..h1 {
            
                // iの右からi2番目のbitが1でない場合, skip.
                // >>は, 右bit shiftを表す.
                // &は, bitwise andを表す.
                if 1 & (i >> i2) == 0 { continue; }

                // jで指定される行ベクトル
                let mut vec_h = vec![];
                
                for j2 in 0..w1 {
                    // jの右からj2番目のbitが1であるか.
                    if 1 & (j >> j2) == 1 {
                        vec_h.push(a[i2][j2]);
                    }
                }

                vec_all.push(vec_h);
            }

            // すべての要素が一致しているか一括確認可能.
            // vecとarrayの直接比較は可能.
            if vec_all == b {
                ans = "Yes";
            }

            // println!("a{:?}", vec_all);
            // println!("b{:?}", b);

        }
    }

    println!("{}", ans);

}