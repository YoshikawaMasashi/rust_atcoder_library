pub fn read_ints() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim_right().to_owned();
    let ws = s.split_whitespace();
    let mut ints = Vec::new();
    for num_s in ws {
        ints.push(num_s.parse().unwrap());
    }
    ints
}
