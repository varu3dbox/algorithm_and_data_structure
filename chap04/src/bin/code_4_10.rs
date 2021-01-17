use proconio::input;

fn func(i: u64, w: u64, a: &[u64]) -> bool {
    if i == 0 {
        if w == 0 {
            return true;
        }
        return false;
    }

    if func(i - 1, w, a) {
        return true;
    };

    if func(i - 1, w - a[(i - 1) as usize], a) {
        return true;
    }

    return false;
}

fn main() {
    // 入力を受け取る
    // input! {
    //     N: u64,
    //     W: u64,
    //     a: [u64; N],
    // }
    let N: u64 = 4;
    let W: u64 = 10;
    let a: [u64; 4] = [3, 2, 6, 5];

    if func(N, W, &a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
