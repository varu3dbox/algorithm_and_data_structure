use proconio::input;

const INF: i64 = 1000000;

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
    let mut dp = vec![INF; N];

    println!("{:?}", rec(N - 1, &h, &mut dp));
}

fn rec(i: usize, h: &[i64], dp: &mut Vec<i64>) -> i64 {
    if dp[i] < INF {
        return dp[i];
    }

    return match i {
        0 => 0,
        _ => {
            let mut res = INF;
            chmin(&mut res, rec(i - 1, &h, dp) + (h[i] - h[i - 1]).abs());

            if i > 2 {
                chmin(&mut res, rec(i - 2, &h, dp) + (h[i] - h[i - 2]).abs());
            }

            dp[i] = res.clone();
            return res;
        }
    };
}
