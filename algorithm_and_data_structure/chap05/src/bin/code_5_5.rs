use proconio::input;

fn chmin(a: &mut i64, b: i64) {
    if *a > b {
        *a = b;
    }
}

fn main() {
    // 入力
    // input! {
    //     N: usize,
    //     h: [i64; N],
    // }
    let N: usize = 7;
    let h: [i64; 7] = [2, 9, 4, 5, 1, 6, 10];

    println!("{:?}", rec(N - 1, &h));
}

fn rec(i: usize, h: &[i64]) -> i64 {
    return match i {
        0 => 0,
        _ => {
            // 十分に大きな値
            let mut res = 100000;
            chmin(&mut res, rec(i - 1, &h) + (h[i] - h[i - 1]).abs());

            if i > 2 {
                chmin(&mut res, rec(i - 2, &h) + (h[i] - h[i - 2]).abs());
            }

            res
        }
    };
}
