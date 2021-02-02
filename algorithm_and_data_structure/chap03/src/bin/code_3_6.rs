use proconio::input;

fn main() {
    const INF: i32 = 20000000;
    // 入力を受け取る
    input! {
        N: usize,
        W: usize,
        a: [i32; N],
    }

    let mut exist: bool = false;
    for bit in 0..(1 << N) {
        let mut sum = 0;
        for i in 0..N {
            if (bit & (1 << i)) != 0 {
                sum += a[i]
            }
        }

        if sum == W as i32 {
            exist = true
        }
    }

    match exist {
        true => println!("Yes"),
        false => println!("No"),
    }
}
