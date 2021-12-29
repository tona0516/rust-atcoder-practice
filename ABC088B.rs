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
    let _n = read::<u32>();
    let mut a = read_vec::<u32>();

    a.sort();
    a.reverse();

    let mut alice = 0;
    let mut bob = 0;
    for (i, _a) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += _a;
        } else {
            bob += _a;
        }
    }

    println!("{}", alice - bob);
}
