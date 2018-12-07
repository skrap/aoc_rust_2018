extern crate regex;
use regex::Regex;

struct MDRingIter {
    center: (i32, i32),
    next: (i32, i32),
    phase: u8,
}

impl MDRingIter {
    fn new(center: (i32, i32), radius: i32) -> MDRingIter {
        if radius == 0 {
            MDRingIter {center, next: center, phase: 8u8}
        } else {
            let next = (center.0, center.1-radius);
            MDRingIter {center, next, phase: 0u8}
        }
    }
}

impl Iterator for MDRingIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<(i32,i32)> {
        match self.phase {
            0 => {
                let result = self.next;
                self.next = (self.next.0-1, self.next.1+1);
                if self.next.1 == self.center.1 {
                    self.phase += 1;
                }
                Some(result)
            }
            1 => {
                let result = self.next;
                self.next = (self.next.0+1, self.next.1+1);
                if self.next.0 == self.center.0 {
                    self.phase += 1;
                }
                Some(result)
            }
            2 => {
                let result = self.next;
                self.next = (self.next.0+1, self.next.1-1);
                if self.next.1 == self.center.1 {
                    self.phase += 1;
                }
                Some(result)
            }
            3 => {
                let result = self.next;
                self.next = (self.next.0-1, self.next.1-1);
                if self.next.0 == self.center.0 {
                    self.phase += 1;
                }
                Some(result)
            }
            8 => { self.phase += 1; Some(self.next) }
            _ => None
        }
    }
}

fn main() {
   let input = include_str!("6_input");
//     let input = "1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9";

    let mut positions = std::collections::HashSet::new();
    let pos_re = Regex::new(r"(\d+), (\d+)").unwrap();
    for line in input.lines() {
        let caps = pos_re.captures(line).unwrap();
        let x : i32 = caps[1].parse().unwrap();
        let y : i32 = caps[2].parse().unwrap();

        positions.insert((x,y));
    }

    let max_x = positions.iter().max_by(|a,b| a.0.cmp(&b.0)).unwrap().0 as i32;
    let max_y = positions.iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap().1 as i32;
    
    let mut areas : std::collections::HashMap<(i32,i32), i32> = std::collections::HashMap::new();
    let mut infinites : std::collections::HashSet<(i32,i32)> = std::collections::HashSet::new();

    for y in 0..=max_y {
        println!("y: {}", y);
    'spot: for x in 0..=max_x {
            for radius in 0.. {
                let closest : Vec<_> = MDRingIter::new((x,y), radius).filter(|c| positions.contains(&c)).collect();
                match closest.len() {
                    0 => continue,
                    1 => {
                        //println!("{:?} is closest to {:?}", (x,y), closest[..].last().unwrap());
                        let winner = closest[..].last().unwrap();
                        let new_val = areas.get(&winner).unwrap_or(&0)+1;
                        areas.insert(*winner, new_val);
                        if x == 0 || x == max_x || y == 0 || y == max_y {
                            infinites.insert(*winner);
                        }
                        continue 'spot
                    },
                    _ => {
                        //println!("{:?} is tied between {:?}", (x,y), closest);
                        // tied.
                        continue 'spot
                    },
                }
            }
        }
    }
    println!("{:?}", infinites);
    let mut results = areas.iter().filter(|(&k,&_v)| ! infinites.contains(&k)).collect::<Vec<_>>();
    results.sort_by(|a,b| a.1.cmp(&b.1));
    println!("{:?}", results);
}