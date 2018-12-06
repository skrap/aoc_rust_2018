
fn main() {
    let input = include_str!("2a_input");
    let mut pair_lines = 0;
    let mut trio_lines = 0;
    for line in input.lines() {
        let mut counts = [0u8; 256];
        for c in line.chars() {
            let idx = c as i32 - 'a' as i32;
            counts[idx as usize] += 1;
        }
        // println!("{}", counts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
        if let Some(_) = counts.iter().position(|x| *x == 2u8) {
            pair_lines += 1;
        }
        if let Some(_) = counts.iter().position(|x| *x == 3u8) {
            trio_lines += 1;
        }
    }
    println!("pair: {}, trio: {}, checksum: {}", pair_lines, trio_lines, pair_lines*trio_lines);
}