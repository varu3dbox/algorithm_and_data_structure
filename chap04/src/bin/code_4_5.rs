fn fibo(N: i32) -> i32 {
    if N == 0 {
        return 0;
    } else if N == 1 {
        return 1;
    }

    return fibo(N - 1) + fibo(N - 2);
}
