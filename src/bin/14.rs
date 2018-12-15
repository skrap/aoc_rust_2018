


fn main() {
    let input = 894501;
    let mut elf1 = 0_usize;
    let mut elf2 = 1_usize;
    let mut scores = vec![3u8, 7u8];
    while scores.len() < input + 10 {
        let mut new_score = scores[elf1] + scores[elf2];
        if new_score >= 10 {
            scores.push(1);
            new_score -= 10;
        }
        scores.push(new_score);
        elf1 += scores[elf1] as usize + 1;
        elf1 %= scores.len();
        elf2 += scores[elf2] as usize + 1;
        elf2 %= scores.len();
    }
    let next_10 : Vec<_> = scores[scores.len()-10..].iter().collect();
    println!("{:?}", next_10);
}