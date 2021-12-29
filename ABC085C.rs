fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn search(n: u32, y: u32) -> Vec<i32> {
    for yukichi in 0..=(y / 10000) {
        for higuchi in 0..=(y / 5000) {
            let noguchi: i32 = (n as i32) - (yukichi as i32) - (higuchi as i32);
            if noguchi < 0 {
                break;
            }
            
            if 10000 * yukichi + 5000 * higuchi + 1000 * (noguchi as u32) == y {
                return vec![yukichi as i32, higuchi as i32, noguchi];
            }
        }
    }

    return vec![-1, -1, -1];
}

fn main() {
    let input = read_vec::<u32>();
    let bill = search(input[0], input[1]);

    println!("{} {} {}", bill[0], bill[1], bill[2]);
}
