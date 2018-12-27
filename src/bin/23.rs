extern crate regex;
use std::collections::{BinaryHeap};

type Coord = i64;

#[derive(Debug,Default,Clone,Ord,PartialOrd,Eq,PartialEq)]
struct Region {
    z: Coord,
    y: Coord,
    x: Coord,
    radius: Coord,
}

#[derive(Debug,Copy,Clone)]
struct Box {
    x: (Coord,Coord),
    y: (Coord,Coord),
    z: (Coord,Coord),
}

fn intersect(a: &(Coord,Coord), b: &(Coord, Coord)) -> Option<(Coord,Coord)> {
    let min = a.0.max(b.0);
    let max = b.1.min(b.1);
    if min <= max {
        Some((min,max))
    } else {
        None
    }
}

fn union(a: &(Coord,Coord), b: &(Coord, Coord)) -> (Coord,Coord) {
    let min = a.0.min(b.0);
    let max = b.1.max(b.1);
    (min, max)
}

fn divide(a: &(Coord,Coord)) -> [Option<(Coord,Coord)>;2] {
    let mid = a.0 + (a.1-a.0+1)/2;
    if mid >= a.1 {
        [Some((a.0,a.1)),None]
    } else {
        [Some((a.0,mid-1)), Some((mid,a.1))]
    }
}

impl Box {
    fn intersect(&self, other: &Box) -> Option<Box> {
        match (intersect(&self.x, &other.x), intersect(&self.y, &other.y), intersect(&self.z, &other.z)) {
            (Some(x),Some(y),Some(z)) => Some(Box { x, y, z }),
            _ => None
        }
    }
    fn union(&self, other: &Box) -> Box {
        Box { x: union(&self.x,&other.x), y: union(&self.y, &other.y), z: union(&self.z, &other.z) }
    }
    fn split(&self) -> [Option<Box>;8] {
        let mut result : [Option<Box>;8] = [None;8];
        let mut i = 0;
        for optx in &divide(&self.x) {
            for opty in &divide(&self.y) {
                for optz in &divide(&self.z) {
                    result[i] = if let (Some(x), Some(y), Some(z)) = (optx,opty,optz) {
                        Some(Box {x:*x, y:*y, z:*z})
                    } else {
                        None
                    };
                    i += 1;
                }
            }
        }
        result
    }
}

impl Region {
    fn new(x: Coord, y: Coord, z: Coord, radius: Coord) -> Region {
        Region {x,y,z, radius}
    }

    fn dist_to(&self, other: &Region) -> Coord {
        (self.x-other.x).abs() + 
        (self.y-other.y).abs() +
        (self.z-other.z).abs()
    }

    fn bbox(&self) -> Box {
        Box { 
            x: (self.x-self.radius, self.x+self.radius),
            y: (self.y-self.radius, self.y+self.radius),
            z: (self.z-self.radius, self.z+self.radius),
        }
    }

    fn union(&self, other: &Self) -> Self {
        Region { radius: self.radius + self.dist_to(other) + other.radius, ..*self }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.dist_to(&other) <= self.radius+other.radius
    }

    fn divide(&self) -> Vec<Region> {
        if self.radius > 1 {
            let delta_c = self.radius/2;
            let new_r = self.radius-delta_c;
            // divide into 6 overlapping volumes.
            let mut result = Vec::with_capacity(6);
            result.push(Region { x: self.x+delta_c, radius: new_r, ..*self });
            result.push(Region { x: self.x-delta_c, radius: new_r, ..*self });
            result.push(Region { y: self.y+delta_c, radius: new_r, ..*self });
            result.push(Region { y: self.y-delta_c, radius: new_r, ..*self });
            result.push(Region { z: self.z+delta_c, radius: new_r, ..*self });
            result.push(Region { z: self.z-delta_c, radius: new_r, ..*self });
            result
        } else if self.radius == 1 {
            // divide into 7 points.
            let mut result = Vec::with_capacity(7);
            result.push(Region { radius: 0, ..*self });
            result.push(Region { x: self.x+1, radius: 0, ..*self });
            result.push(Region { x: self.x-1, radius: 0, ..*self });
            result.push(Region { y: self.y+1, radius: 0, ..*self });
            result.push(Region { y: self.y-1, radius: 0, ..*self });
            result.push(Region { z: self.z+1, radius: 0, ..*self });
            result.push(Region { z: self.z-1, radius: 0, ..*self });
            result
        } else {
            panic!("tried to divide a unit volume");
        }
    }

    fn is_point(&self) -> bool {
        self.radius == 0
    }
}


fn search(start_bots: Vec<Region>) -> (usize, Region) {
    let mut best_point_count = 2usize;
    let mut best_point = Region { ..Default::default() };
    let origin = Region { ..Default::default() };
    let mut boxes : BinaryHeap<(usize, Region, Vec<Region>)> = BinaryHeap::new();
    for bot in start_bots.clone() {
        let mut start_region_bots : Vec<_> = start_bots.iter().filter(|b| b.intersects(&bot)).map(|b| b.clone()).collect();
        boxes.push((start_region_bots.len(), bot.clone(), start_region_bots.clone()));
    }
    while let Some((score,region,bots)) = boxes.pop() {
        assert!(score == bots.len());
        if bots.len() < best_point_count {
            continue;
        }
        if region.is_point() {
            if bots.len() > best_point_count || 
            (bots.len() == best_point_count && region.dist_to(&origin) < best_point.dist_to(&origin)) {
                best_point_count = bots.len();
                best_point = region.clone();
                println!("new best count {} for point: {:?}", best_point_count, best_point);
            }
        } else {
            let divisions = region.divide();
            for subregion in divisions.into_iter() {
                let mut subregion_bots = Vec::new();
                for r in bots.iter().filter(|b| b.intersects(&subregion)) {
                    subregion_bots.push(r.clone());
                }
                if subregion_bots.len() >= best_point_count {
                    boxes.push((subregion_bots.len(),subregion,subregion_bots));
                }
            }
        }
    }
    (best_point_count,best_point)
}

fn main() {
    let input = include_str!("23_input");
    let re = regex::Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut bots = Vec::new();
    for caps in re.captures_iter(input) {
        bots.push(Region{
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
            z: caps[3].parse().unwrap(),
            radius: caps[4].parse().unwrap(),
        });
    }
    bots.sort_by(|a,b| a.radius.cmp(&b.radius));

    let strongest = bots.iter().max_by(|a,b| a.radius.cmp(&b.radius)).unwrap().clone();
    let in_range = bots.iter().filter(|b| strongest.dist_to(b) <= strongest.radius).count();
    println!("strongest: {:?}: in range count {}", strongest, in_range);

    let (size, point) = search(bots);
    println!("size: {} point {:?} dist: {}", size, &point, point.dist_to(&Region::new(0, 0, 0, 0)));
}