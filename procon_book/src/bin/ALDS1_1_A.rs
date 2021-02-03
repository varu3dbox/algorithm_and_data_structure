use proconio::input;
use std::cmp;

fn main() {
    //    input! {
    //        n: usize,
    //        R: [i32; n],
    //    }
    let n: usize = 6;
    let mut source: Vec<i32> = vec![5, 2, 4, 6, 1, 3];

    for i in 1..source.len() {
        let mut j: usize = i;

        while j > 0 && source[j - 1] > source[j] {
            source.swap(j - 1, j);
            j -= 1;
        }

        println!("{:?}", source);
    }
}
