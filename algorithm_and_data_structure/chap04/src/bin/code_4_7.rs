fn main() {
    let mut F = vec![0, 1];
    for N in 2..50 {
        F.push(F[N - 1] + F[N - 2]);
        println!("{:?}項目: {:?}", N, F[N])
    }
}
