fn fibo(N: i32) -> i32 {
    println!("fibo({:?})を呼び出しました", N);

    if N == 0 {
        return 0;
    } else if N == 1 {
        return 1;
    }

    let mut result = fibo(N - 1) + fibo(N - 2);
    println!("{:?}項目 = {:?}", N, result);

    return result;
}

fn main() {
    fibo(6);
}
