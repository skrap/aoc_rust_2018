use std::collections::HashMap;

#[derive(Clone,PartialEq,Eq,Hash)]
enum Tile {
    Open,
    Trees,
    Lumber,
}

#[derive(Hash,Eq,PartialEq,Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(input: &[u8]) -> Map {
        let input_width = input.iter().position(|b| *b == b'\n').unwrap();
        let width = input_width+2;
        let mut tiles = Vec::new();
        let mut height = 1;
        tiles.resize(width, Tile::Open);
        for row in input.chunks_exact(input_width+1) {
            height += 1;
            tiles.push(Tile::Open);
            for byte in row {
                tiles.push(match byte {
                    b'.' => Tile::Open,
                    b'|' => Tile::Trees,
                    b'#' => Tile::Lumber,
                    b'\n' => Tile::Open,
                    _ => panic!("Unknown byte: {}", byte),
                });
            }
        }
        tiles.extend((0..width).into_iter().map(|_n| Tile::Open));
        height += 1;
        Map {width, height, tiles}
    }
    
    fn tick(&mut self) {
        let mut next = Vec::new();
        next.extend((0..self.width).into_iter().map(|_n| Tile::Open));
        for row in 1..self.height-1 {
            let get = |x,y| {
                let t : &Tile = &self.tiles[y*self.width+x];
                t.clone()
            };
            next.push(Tile::Open);
            for col in 1..self.width-1 {
                let surrounding = vec!(
                    get(col-1,row-1),get(col,row-1),get(col+1,row-1),
                    get(col-1,row),                 get(col+1,row),
                    get(col-1,row+1),get(col,row+1),get(col+1,row+1)
                );

                next.push(match get(col,row) {
                    /* 
                    An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
                    */
                    Tile::Open => {
                        if surrounding.iter().filter(|t| **t == Tile::Trees).count() >= 3 {
                            Tile::Trees
                        } else {
                            Tile::Open
                        }
                    },

                    /*
                    An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
                    */
                    Tile::Trees => {
                        if surrounding.iter().filter(|t| **t == Tile::Lumber).count() >= 3 {
                            Tile::Lumber
                        } else {
                            Tile::Trees
                        }
                    },

                    /*
                    An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
                    */
                    Tile::Lumber => {
                        if surrounding.iter().filter(|t| **t == Tile::Lumber).count() >=1 &&
                        surrounding.iter().filter(|t| **t == Tile::Trees).count() >= 1 {
                            Tile::Lumber
                        } else {
                            Tile::Open
                        }
                    },
                });

            }
            next.push(Tile::Open);
        }
        next.extend((0..self.width).into_iter().map(|_n| Tile::Open));
        assert!(next.len() == self.tiles.len());
        self.tiles = next;
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.tiles[..].chunks_exact(self.width) {
            write!(f, "{}\n", row.iter().map(|t| match t {
                Tile::Open => '.',
                Tile::Lumber => '#',
                Tile::Trees => '|',
            }).collect::<String>())?;
        }
        Ok(())
    }
}

fn main() {
    print!("\x1b\x5b2J");

    let input = include_bytes!("18_input");
    let mut map = Map::new(input);
    let mut stdout = std::io::stdout();
    println!("{}", map);
    let mut seen : HashMap<Map,usize> = HashMap::new();
    let mut t = 0;
    let iters = 1000000000;
    while t < iters {
        map.tick();
        t += 1;
        if seen.contains_key(&map) {
            println!("tick {} == tick {}", t, seen[&map]);
            let stride = t - seen[&map];
            while t + stride < 1000000000 {
                t += stride;
            }
        }
        seen.insert(map.clone(),t);
        if true {
            print!("\x1b\x5b;H");
            println!("{}", map);
            let ten_millis = std::time::Duration::from_millis(10);
            std::thread::sleep(ten_millis);
        }
    }

    let woods = map.tiles.iter().filter(|t| **t == Tile::Trees).count();
    let lumber = map.tiles.iter().filter(|t| **t == Tile::Lumber).count();
    println!("woods: {}, lumber: {}, answer: {}", woods, lumber, woods*lumber);
}