fn main() {
    println!("{:?}", func(5));
}

fn func(N: i32) -> i32 {
    if N == 0 {
        return 0;
    }

    return N + func(N - 1);
}
