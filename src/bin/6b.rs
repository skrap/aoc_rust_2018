extern crate regex;

const DIST_LIMIT : i32 = 10_000;

struct Pt {
    x: i32,
    y: i32
}

impl Pt {
    fn dist_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
   let input = include_str!("6_input");
    
    let mut positions = Vec::new();
    let pos_re = regex::Regex::new(r"(\d+), (\d+)").unwrap();
    for line in input.lines() {
        let caps = pos_re.captures(line).unwrap();
        let x : i32 = caps[1].parse().unwrap();
        let y : i32 = caps[2].parse().unwrap();

        positions.push(Pt{x,y});
    }

    let max_x = positions.iter().max_by(|a,b| a.x.cmp(&b.y)).unwrap().x as i32;
    let max_y = positions.iter().max_by(|a,b| a.y.cmp(&b.y)).unwrap().y as i32;

    let mut area = 0u32;
    for y in 0..=max_y {
        println!("y: {}", y);
        for x in 0..=max_x {
            let this_p = Pt{x,y};
            let total_dist : i32 = positions.iter().map(|p| this_p.dist_to(p)).sum();
            if total_dist < DIST_LIMIT {
                area += 1;
            }
        }
    }
    println!("area: {}", area);
}