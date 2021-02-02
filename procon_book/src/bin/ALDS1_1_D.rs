use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        R: [i32; n],
    }
    let mut maxv: i33 = std::i32::MIN;
    let mut minv: i32 = R[0];

    for j in (1..n) {
        maxv = cmp::max(maxv, R[j] - minv);
        minv = cmp::min(minv, R[j]);
    }
    println!("{:?}", maxv)
}
