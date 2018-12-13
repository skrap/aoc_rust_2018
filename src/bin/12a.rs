extern crate regex;

const INITIAL_STATE : &'static str = "..#..###...#####.#.#...####.#..####..###.##.#.#.##.#....#....#.####...#....###.###..##.#....#######";
const RULES : &'static str = 
"..### => .
.##.# => #
#..#. => .
#.#.# => #
###.. => #
.#..# => .
##..# => #
.###. => #
..#.. => .
..... => .
##### => .
.#... => #
...#. => #
#...# => #
####. => .
.#### => .
##.## => #
...## => .
..##. => .
#.##. => .
#.... => .
.#.#. => .
..#.# => #
#.#.. => #
##... => #
##.#. => .
#..## => .
.##.. => .
#.### => .
....# => .
.#.## => #
###.# => #";

struct Rule {
    pat: [bool;5], 
    result: bool,
}

impl Rule {
    fn new(text: &str) -> Rule {
        let chars : Vec<_> = text.chars().collect();
        assert!(chars.len() == 10);
        let mut pat = [false;5];
        for i in 0..5 {
            pat[i] = chars[i] == '#';
        }
        let result = *chars.last().unwrap() == '#';
        Rule { pat, result }
    }
}

fn sum_pots(pots: &[bool]) -> i64 {
    let mut sum = 0;
    for idx in 0..pots.len() {
        let value = idx as i32 - ZERO_POT_IDX as i32;
        if pots[idx] {
            sum += value as i64;
        }
    }
    sum
}

fn print_pots(pots: &[bool]) {
    for pot in pots {
        print!("{}", if *pot { '#' } else { '.' });
    }
    print!("\n");
}

const GENS : usize = 20;
const ZERO_POT_IDX : usize = GENS;


fn main() {
    let mut rules = Vec::new();
    for line in RULES.lines() {
        rules.push(Rule::new(line));
    }
    let mut pots : Vec<bool> = Vec::new();
    pots.resize(INITIAL_STATE.len() + (GENS+6)*2, false);
    for (i,c) in INITIAL_STATE.chars().enumerate() {
        pots[ZERO_POT_IDX+i] = c == '#';
    }

    rules.sort_by(|a,b| a.pat.cmp(&b.pat));

    for _gen in 0..GENS {
        let mut next_pots = Vec::new();
        next_pots.extend_from_slice(&[false;2]);
        for w in pots.windows(5) {
            if let Ok(rule_idx) = rules.binary_search_by(|a| w.cmp(&a.pat).reverse()) {
                let rule = &rules[rule_idx];
                next_pots.push(rule.result);
            } else {
                next_pots.push(false);
            }
        }
        next_pots.extend_from_slice(&[false;2]);
        pots = next_pots;
        print_pots(&pots[..]);
    }

    println!("sum {}", sum_pots(&pots));
}