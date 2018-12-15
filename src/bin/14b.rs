
fn main() {
    let target = [8u8, 9u8, 4u8, 5u8, 0u8, 1u8];
    let mut elf1 = 0_usize;
    let mut elf2 = 1_usize;
    let mut scores = vec![3u8, 7u8];
    loop {
        let mut new_score = scores[elf1] + scores[elf2];
        if new_score >= 10 {
            scores.push(1);
            new_score -= 10;

            if scores.len() >= 6 && scores[scores.len()-6..] == target { break; }
        }
        scores.push(new_score);
        if scores.len() >=6 && scores[scores.len()-6..] == target { break; }

        elf1 += scores[elf1] as usize + 1;
        elf1 %= scores.len();
        elf2 += scores[elf2] as usize + 1;
        elf2 %= scores.len();
    }
    println!("recipes before target: {}", scores.len()-target.len());
}