fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let parts: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let a = parts[0];
    let b = parts[1];

    println!("{}\n{}\n{}", a + b, a - b, a * b);
}
