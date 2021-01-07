use proconio::input;
use std::io::Read;

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
}

fn main() {
    // 入力データを受け取る
    input! {
        n: usize,
        points: [[f64; 2]; n]
    }
    let x: Vec<f64> = points.clone().into_iter().map(|p| p[0]).collect();
    let y: Vec<f64> = points.clone().into_iter().map(|p| p[1]).collect();

    // 求める値を十分大きい値で初期化する
    let mut minimum_dist: f64 = 100000000.0;

    // 探索開始
    for i in 0..n {
        for j in (i + 1)..n {
            let dist_i_j = calc_dist(x[i], y[i], x[j], y[j]);
            if dist_i_j < minimum_dist {
                minimum_dist = dist_i_j;
            }
        }
    }

    println!("{:?}", minimum_dist)
}
