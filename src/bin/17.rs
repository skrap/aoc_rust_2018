use std::collections::BTreeSet;
use std::cmp::{min, max};
use std::fmt;
extern crate regex;

#[derive(Clone,PartialEq)]
enum Tile {
    Sand,
    Clay,
    FlowingWater,
    StillWater,
}

impl Tile {
    fn solid(&self) -> bool {
        self == &Tile::Clay || self == &Tile::StillWater
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos {x, y}
    }
    fn offset(&self, x: i32, y: i32) -> Pos {
        Pos { x: self.x+x, y: self.y+y }
    }
    fn above(&self) -> Pos {
        self.offset(0,-1)
    }
    fn below(&self) -> Pos {
        self.offset(0,1)
    }
    fn left(&self) -> Pos {
        self.offset(-1,0)
    }
    fn right(&self) -> Pos {
        self.offset(1, 0)
    }
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    topleft: Pos,
}

impl Map {
    fn new(xbounds: &(i32, i32), ybounds: &(i32, i32)) -> Map {
        let width = (xbounds.1 - xbounds.0) as usize + 1 + 2;
        let height = ybounds.1 as usize + 2; // +2 for lookahead
        let mut map = Map { tiles: Vec::new(), width, height, topleft: Pos {x: xbounds.0-1, y: 0 } };
        map.tiles.resize(width*height, Tile::Sand);
        map
    }

    fn get(&self, pos: &Pos) -> Tile {
        let x = (pos.x - self.topleft.x) as usize;
        let y = (pos.y - self.topleft.y) as usize;
        self.tiles[y*self.width+x].clone()
    }

    fn set(&mut self, pos: &Pos, tile: Tile) {
        let x = (pos.x - self.topleft.x) as usize;
        let y = (pos.y - self.topleft.y) as usize;
        self.tiles[y*self.width+x] = tile;
    }

    fn run(&mut self, source: &Pos) {
        use std::collections::VecDeque;
        let mut heads : VecDeque<Pos> = VecDeque::new();
        heads.push_back(source.clone());
        let mut max = 500000;
        while let Some(head) = heads.pop_front() {
            max -= 1;
            if max == 0 { break; } 
            if self.get(&head) == Tile::FlowingWater { continue; }
            if head.y as usize >= self.height-1 { continue; } // drip has dropped off the bottom.

            if !self.get(&head.below()).solid() {
                self.set(&head, Tile::FlowingWater);
                heads.push_back(head.below());
            } else {
                // water wants to drip down, but can't.  go left and right.
                // look in either direction
                // stop if wall or space below
                let mut left = head.clone();
                while self.get(&left.below()).solid() && !self.get(&left.left()).solid() {
                    left = left.left();
                }
                let mut right = head.clone();
                while self.get(&right.below()).solid() && !self.get(&right.right()).solid() {
                    right = right.right();
                }
                if self.get(&left.below()).solid() && self.get(&right.below()).solid() {
                    // we're in a cup. fill with still water.
                    while left <= right {
                        self.set(&left, Tile::StillWater);
                        left = left.right();
                    }
                    // the drip is still dripping
                    self.set(&head.above(), Tile::Sand);
                    heads.push_back(head.above());
                } else {
                    if !self.get(&left.below()).solid() {
                        heads.push_back(left.below());
                    }
                    if !self.get(&right.below()).solid() {
                        heads.push_back(right.below());
                    }
                    // fill will flowing water.
                    while left <= right {
                        self.set(&left, Tile::FlowingWater);
                        left = left.right();
                    }
                }
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut y = 0;
        while y < self.tiles.len() {
            writeln!(f, "{}", (y..y+self.width).map(|i| match self.tiles[i] {
                Tile::Clay => '#',
                Tile::Sand => '.',
                Tile::FlowingWater => '|',
                Tile::StillWater => '~',
            }).collect::<String>())?;
            y += self.width;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("17_input");
    let mut clay : BTreeSet<Pos> = BTreeSet::new();
    let re = regex::Regex::new(r"(\w)=(\d+), \w=(\d+)\.\.(\d+)").unwrap();
    for caps in re.captures_iter(input) {
        let primary_dim = &caps[1];
        let primary_coord : i32 = caps[2].parse().unwrap();
        let range = (caps[3].parse().unwrap())..=(caps[4].parse().unwrap());
        if primary_dim == "x" {
            for y in range {
                clay.insert(Pos::new(primary_coord, y));
            }
        } else {
            for x in range {
                clay.insert(Pos::new(x, primary_coord));
            }
        }
    }
    let xbounds = clay.iter().fold((i32::max_value(), i32::min_value()), |acc, k| (min(acc.0,k.x), max(acc.1,k.x)));
    let ybounds = clay.iter().fold((i32::max_value(), i32::min_value()), |acc, k| (min(acc.0,k.y), max(acc.1,k.y)));
    
    let mut map = Map::new(&xbounds, &ybounds);
    for pos in clay {
        map.set(&pos, Tile::Clay);
    }

    map.run(&Pos::new(500,0));
    println!("{}", map);
    
    println!("water spaces: {}", map.tiles.iter().skip(map.width*ybounds.0 as usize).filter(|t| **t == Tile::FlowingWater || **t == Tile::StillWater).count());
    println!("still water spaces: {}", map.tiles.iter().skip(map.width*ybounds.0 as usize).filter(|t| **t == Tile::StillWater).count());
}