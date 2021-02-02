fn chmax(a: &mut usize, b: usize) {
    if *a < b {
        *a = b;
    }
}

fn main() {
    let n: usize = 6;
    let max_w: usize = 9;
    let product = vec![(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)];

    let mut dp = vec![vec![0; max_w + 1]; n + 1];

    for i in (0..n) {
        for w in (0..=max_w) {
            // i 番目の品物を選ぶ場合
            if w >= product[i].0 {
                let tmp = dp[i][w - product[i].0] + product[i].1;
                chmax(&mut dp[i + 1][w], tmp);
            }
            let tmp = dp[i][w];
            chmax(&mut dp[i + 1][w], tmp);
        }
    }
    println!("{:?}", dp[n][max_w]);
}
