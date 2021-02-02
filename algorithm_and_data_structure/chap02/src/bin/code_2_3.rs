fn main() {
    let mut N = String::new();
    std::io::stdin().read_line(&mut N).ok();

    let max_num = N.trim().parse().ok().unwrap();
    let mut num = 2;
    while num < max_num {
        println!("{}", num);
        num += 2;
    }
}