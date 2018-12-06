use std::collections::BTreeSet;

fn main() {
    let input = include_str!("1a_input");
    let mut seen : BTreeSet<i32> = BTreeSet::new();
    let mut freq = 0_i32;
    'a: loop {
        for line in input.lines() {
            if seen.contains(&freq) {
                println!("first repeat: {}", freq);
                break 'a;
            }
            seen.insert(freq);

            let i : i32 = line.parse().unwrap();
            freq += i;
        }
    }
    println!("{}\n", freq);
}
