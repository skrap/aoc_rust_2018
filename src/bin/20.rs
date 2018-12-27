use std::collections::{VecDeque,BTreeSet,BTreeMap};

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Hash,Debug)]
struct Pos {
    y: i32,
    x: i32
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
    fn offset(&self, x: i32, y: i32) -> Pos {
        Pos {x: self.x + x, y: self.y + y}
    }
}

#[derive(Clone,Debug)]
struct Room {
    n: bool,
    e: bool,
    w: bool,
    s: bool,
    dist: i32,
}

impl Room {
    fn new() -> Room {
        Room { 
            n: false,
            e: false,
            w: false,
            s: false,
            dist: i32::max_value(),
        }
    }
}

fn skip_rest(input: &'static [u8], start: usize) -> usize {
    let mut num_open = 1;
    for i in start.. {
        let b = input[i];
        match b {
            b'(' => {
                num_open += 1;
            },
            b')' => {
                num_open -= 1;
                if num_open == 0 {
                    return i + 1;
                }
            },
            _ => (),
        }
    }
    panic!("couldn't find a close parens for branch starting at {}", start - 1);
}

fn skip_one(input: &'static [u8], start: usize) -> Option<usize> {
    let mut num_open = 1;
    for i in start.. {
        let b = input[i];
        match b {
            b'(' => {
                num_open += 1;
            },
            b')' => {
                num_open -= 1;
                if num_open == 0 {
                    return None;
                }
            },
            b'|' => {
                if num_open == 1 {
                    return Some(i+1);
                }
            }
            _ => (),
        }
    }
    panic!("couldn't find a close parens for branch starting at {}", start - 1);
}

fn find_distances(map: &mut BTreeMap<Pos,Room>) {
    let mut heads = VecDeque::new();
    heads.push_back((Pos::new(0, 0), 0));
    while let Some((pos, dist)) = heads.pop_front() {
        let mut room = map.get_mut(&pos).unwrap();
        if dist < room.dist { // this is a shorter way
            room.dist = dist;
            if room.n { heads.push_back((pos.offset(0,-1),dist+1)) }
            if room.s { heads.push_back((pos.offset(0, 1),dist+1)) }
            if room.e { heads.push_back((pos.offset(1, 0),dist+1)) }
            if room.w { heads.push_back((pos.offset(-1,0),dist+1)) }
        }
    }
}

fn main() {
    let input = include_bytes!("20_input");
    //let input = b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"; 
    
    let mut heads = Vec::new();
    let mut map : BTreeMap<Pos,Room> = BTreeMap::new();
    heads.push((Pos::new(0, 0), 1));
    map.insert(Pos::new(0, 0), Room::new());

    let mut all_heads = BTreeSet::new();

    let mut routes = 0;

    while let Some((pos,start_idx)) = heads.pop() {
        let mut pos = pos.clone();
        // always assume current room has been entered
        // follow direction noted, adding doors to both rooms.
        let mut i = start_idx;
        loop {
            let b = input[i];
            i += 1;
            match b {
                b'N' => {
                    map.get_mut(&pos).unwrap().n = true;
                    let newpos = pos.offset(0,-1);
                    map.entry(newpos.clone()).or_insert(Room::new()).s = true;
                    pos = newpos;
                },
                b'S' => {
                    map.get_mut(&pos).unwrap().s = true;
                    let newpos = pos.offset(0,1);
                    map.entry(newpos.clone()).or_insert(Room::new()).n = true;
                    pos = newpos;
                },
                b'E' => {
                    map.get_mut(&pos).unwrap().e = true;
                    let newpos = pos.offset(1,0);
                    map.entry(newpos.clone()).or_insert(Room::new()).w = true;
                    pos = newpos;
                },
                b'W' => {
                    map.get_mut(&pos).unwrap().w = true;
                    let newpos = pos.offset(-1,0);
                    map.entry(newpos.clone()).or_insert(Room::new()).e = true;
                    pos = newpos;
                },
                b'(' => {
                    let mut skip_to = i;
                    while let Some(branch_idx) = skip_one(input, skip_to) {
                        let next = (pos.clone(), branch_idx);
                        if ! all_heads.contains(&next) {
                            heads.push(next.clone());
                            all_heads.insert(next.clone());
                        }
                        skip_to = branch_idx;
                    }
                    // continue with pos and i
                },
                b'|' => {
                    i = skip_rest(input, i);
                    // continue.
                },
                b')' => {
                    // continue.
                }
                b'$' => {
                    routes += 1;
                    break;
                },
                _ => panic!("unknown byte {}", b),
            }
        }
    }

    println!("routes: {}, rooms: {}, heads: {:?}", routes, map.len(), heads.len());

    find_distances(&mut map);

    println!("farthest: {:?}", map.iter().max_by(|a,b| a.1.dist.cmp(&b.1.dist)).unwrap());

    println!("at least dist: 1000 {}", map.iter().filter(|(_k,r)| r.dist >= 1000).count());
}