fn main() {
    let mut N = String::new();
    std::io::stdin().read_line(&mut N).ok();

    let max_loop_count = N.trim().parse().ok().unwrap();
    let mut count = 0;

    while count < max_loop_count {
        while count < max_loop_count {
            count += 1
        }
    }
}