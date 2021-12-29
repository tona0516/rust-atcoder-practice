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
    let n = read::<u32>();
    let mut a: Vec<u32> = Vec::new();
    for _i in 0..n {
        a.push(read::<u32>());
    }

    a.sort();
    a.reverse();

    let mut count = 0;
    let mut lower_mochi = 101;
    for _a in a {
        if _a < lower_mochi {
            count += 1;
            lower_mochi = _a;
        }
    }

    println!("{}", count);
}
