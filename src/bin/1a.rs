
fn main() {
    let input = include_str!("1a_input");
    let mut total = 0_i32;
    for line in input.lines() {
        let i : i32 = line.parse().unwrap();
        total += i;
    }
    println!("1a: Hello, world!");
    println!("{}\n", total);
}
