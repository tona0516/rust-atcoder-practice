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

fn is_even_all(a: &Vec<i32>) -> bool {
    a.iter().filter(|&x| x % 2 == 0).count() == a.len()
}

fn main() {
    let _n = read::<i32>();
    let mut a = read_vec::<i32>();

    let mut count: i32 = 0;
    loop {
        if is_even_all(a.as_ref()) {
            count += 1;
            a = a.iter().map(|x| x / 2).collect();
            continue;
        }
        
        break;
    }
    
    println!("{}", count)
}