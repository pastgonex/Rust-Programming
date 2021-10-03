use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();
    println!("{}", n);
}
