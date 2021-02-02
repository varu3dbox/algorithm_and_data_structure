use proconio::input;
use std::io::Read;

fn main() {
    // 入力を受け取る
    input! {
        N: usize,
        a: [i64; N],
        v: i64
    }

    // 線形探索
    let mut exist: bool = false;
    for i in 0..N {
        if a[i] == v {
            exist = true
        }
    }

    // 結果出力
    match exist {
        true => println!("Yes"),
        false => println!("No"),
    }
}
