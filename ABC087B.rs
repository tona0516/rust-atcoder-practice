fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let a = read::<i32>();
    let b = read::<i32>();
    let c = read::<i32>();
    let x = read::<i32>();

    let mut count: i32 = 0;
    for _a in (0..=a) {
        for _b in (0..=b) {
            for _c in (0..=c) {
                if 500 * _a + 100 * _b + 50 * _c == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
