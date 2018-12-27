struct Point {
    coords: (i8,i8,i8,i8),
    constl: Option<usize>,
}

impl Point {
    fn dist(&self, other: &Self) -> i8 {
        (self.coords.0-other.coords.0).abs() + 
        (self.coords.1-other.coords.1).abs() + 
        (self.coords.2-other.coords.2).abs() + 
        (self.coords.3-other.coords.3).abs()
    }

    fn joined(&self, other: &Self) -> bool {
        self.dist(other) <= 3
    }
}

fn main() {
    let input = include_str!("25_input");
    let mut points = Vec::new();
    for line in input.lines() {
        let nums : Vec<i8> = line.split(",").map(|p| p.parse().unwrap()).collect();
        points.push(Point{ coords: (nums[0],nums[1],nums[2],nums[3]), constl: None });
    }

    // everything starts as a constellation of its own.
    let mut constls : Vec<Vec<usize>> = Vec::new();
    for pt1 in 0..points.len() {
        constls.push(Vec::from(&[pt1][..]));
    }

    loop {
        let mut any_joins = false;
        for c1_idx in 0..constls.len() {
            // can this constellation join any others?
            for c2_idx in (c1_idx+1)..constls.len() {

                // check c2 if it can join
                let mut joined = false;
                'join: for c1_pt_idx in constls[c1_idx].iter() {
                    for c2_pt_idx in constls[c2_idx].iter() {
                        let pt1 = &points[*c1_pt_idx];
                        let pt2 = &points[*c2_pt_idx];
                        if pt1.joined(pt2) {
                            joined = true;
                            break 'join;
                        }
                    }
                }
                if joined {
                    any_joins = true;
                    println!("constl {} joining {}", c1_idx, c2_idx);
                    let (first,rest) = constls.split_at_mut(c2_idx);
                    first[c1_idx].append(&mut rest[0]);
                }
            }
        }
        if ! any_joins {
            break;
        }
    }
    println!("constellation count: {}", constls.iter().filter(|c| c.len() > 0).count());
}