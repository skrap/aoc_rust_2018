fn main() {
    let input = include_str!("2a_input");
    for line in input.lines() {
        for otherline in input.lines() {
            let mut diffs = 0;
            let mut same_str = String::new();
            for (c1, c2) in line.chars().zip(otherline.chars()) {
                if c1 != c2 {
                    diffs += 1;
                } else {
                    same_str.push(c1);
                }
            }
            if diffs == 1 {
                println!("{}", same_str);
                return;
            }
        }
    }
}
