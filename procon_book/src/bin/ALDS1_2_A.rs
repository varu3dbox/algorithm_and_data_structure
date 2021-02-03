use proconio::input;
use std::cmp;

fn main() {
    //    input! {
    //        n: usize,
    //        R: [i32; n],
    //    }
    let n: usize = 5;
    let mut source: Vec<i32> = vec![5, 3, 2, 4, 1];

    for i in 0..n {
        let mut j = n - 1;
        while j > i {
            if source[j] < source[j - 1] {
                source.swap(j, j - 1);
            }
            j -= 1;
        }
    }

    println! {"{:?}", source}
}
