use proconio::input;
use std::cmp;

fn main() {
    //    input! {
    //        n: usize,
    //        R: [i32; n],
    //    }
    let n: usize = 6;
    let mut source: Vec<i32> = vec![5, 6, 4, 2, 1, 3];

    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if source[j] < source[minj] {
                minj = j;
            }
        }
        source.swap(i, minj);
    }

    println! {"{:?}", source}
}
