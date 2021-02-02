fn main() {
    println!("{:?}", func(5));
}

fn func(N: i32) -> i32 {
    println!("func {:?}を呼び出しました", N);
    if N == 0 {
        return 0;
    }

    let mut result = N + func(N - 1);
    println!("{:?} までの和 = {:?}", N, result);
    return result;
}
