fn main() {
    println!("{:?}", func(5));
}

// stack overflow
fn func(N: i32) -> i32 {
    if N == 0 {
        return 0;
    }

    let mut result = N + func(N + 1);
    return result;
}
