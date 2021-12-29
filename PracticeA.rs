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
    let bc = read_vec::<i32>();
    let s = read::<String>();

    let sum: i32 = a + bc[0] + bc[1];

    println!("{}", sum);
    println!("{}", s);
}
