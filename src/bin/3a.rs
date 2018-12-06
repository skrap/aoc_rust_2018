extern crate regex;
use regex::Regex;

fn main() {
    // Looks like #5 @ 793,21: 23x10
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut fabric = vec![vec![0u8; 1000]; 1000];

    let input = include_str!("3a_input");
    for line in input.lines() {
        let capts = re.captures(line).unwrap();
        let vals : Vec<_> = (2..6).map(|i| capts.get(i).unwrap().as_str().parse::<u32>().unwrap()).collect();
        let x_start = vals[0] as usize;
        let y_start = vals[1] as usize;
        let width = vals[2] as usize;
        let height = vals[3] as usize;
        // println!("{:?}", (x,y,width,height));

        for y in y_start..y_start+height {
            for x in x_start..x_start+width {
                match fabric[x][y] {
                    0 => fabric[x][y] = 1,
                    1 => fabric[x][y] = 2,
                    _ => (),
                 }
            }
        }
    }

    let mut count = 0;
    for a in fabric {
        for b in a {
            if b >= 2 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}