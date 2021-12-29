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
    let input = read_vec::<u32>();
    const RADIX: u32 = 10;
    let mut sum: u32 = 0;

    for n in 1..=input[0] {
        let digit_sum = n.to_string().chars().map(|c| c.to_digit(RADIX).unwrap()).sum::<u32>();
        if digit_sum >= input[1] && digit_sum <= input[2] {
            sum += n;
        }
    }

    println!("{}", sum);
}
