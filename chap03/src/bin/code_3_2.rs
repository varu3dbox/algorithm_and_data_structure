use proconio::input;

fn main() {
    // 入力を受け取る
    input! {
        N: usize,
        a: [i64; N],
        v: i64
    }

    // 線形探索
    let mut found_id: i64 = -1;
    for i in 0..N {
        if a[i] == v {
            found_id = i as i64;
            break;
        }
    }

    // 結果出力
    println!("{:?}", found_id)
}
