use std::collections::HashMap;

fn fibo(N: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    return match memo.get(&N) {
        None => {
            let new_num = fibo(N - 1, memo) + fibo(N - 2, memo);
            memo.insert(N, new_num);
            new_num
        }
        _ => memo[&N],
    };
}

fn main() {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);

    fibo(50, &mut memo);
    println!("{:?}", memo);
}
