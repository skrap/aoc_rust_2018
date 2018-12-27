
#[derive(Clone)]
enum Region {
    Rocky,
    Wet,
    Narrow
}

#[derive(Clone,PartialEq,Eq)]
enum Tools {
    Climb,
    Torch,
    Neither,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Pos {
    y: i32,
    x: i32,
}

type GeoIdx = u64;

#[derive(Clone, Default, Debug)]
struct Tile {
    erosion: Option<GeoIdx>,
    climb_cost: u64,
    torch_cost: u64,
    neither_cost: u64,
}

impl Tile {
    fn cost(&self, tool: &Tools) -> u64 {
        match *tool {
            Tools::Climb => self.climb_cost,
            Tools::Torch => self.torch_cost,
            Tools::Neither => self.neither_cost,
        }
    }

    fn set_cost(&mut self, tool: &Tools, cost: u64) {
        match *tool {
            Tools::Climb => self.climb_cost = cost,
            Tools::Torch => self.torch_cost = cost,
            Tools::Neither => self.neither_cost = cost,
        };
    }
}

#[derive(Default)]
struct Map {
    depth: u64,
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(target: &Pos, height: usize, width: usize) -> Map {
        let mut result = Map { height, width, ..Default::default() };
        result.tiles.resize(height*width, 
            Tile {
                erosion: None, 
                climb_cost: u64::max_value(),
                torch_cost: u64::max_value(), 
                neither_cost: u64::max_value()
            });
        result.tiles[0].erosion = Some(0);
        result[target].erosion = Some(0);
        result
    }
}

impl std::ops::Index<&Pos> for Map {
    type Output = Tile;

    fn index<'a>(&'a self, index: &Pos) -> &'a Tile {
        &self.tiles[(index.y*self.width as i32+index.x) as usize]
    }
}

impl std::ops::IndexMut<&Pos> for Map {
    fn index_mut<'a>(&'a mut self, index: &Pos) -> &'a mut Tile {
        &mut self.tiles[(index.y*self.width as i32+index.x) as usize]
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos {x, y}
    }
    fn offset(&self, x: i32, y: i32) -> Pos {
        Pos { x: self.x+x, y: self.y+y }
    }
    
    fn erosion(&self, map: &mut Map) -> GeoIdx {
        /*
        The region at 0,0 (the mouth of the cave) has a geologic index of 0.
        The region at the coordinates of the target has a geologic index of 0.
        If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
        If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
        Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
        */
        if let Some(erosion) = map[self].erosion {
            return erosion;
        }
        let g = if self.x == 0 && self.y == 0 {
            0 as GeoIdx
        } else if self.y == 0 {
            self.x as GeoIdx * 16807 
        } else if self.x == 0 {
            self.y as GeoIdx * 48271 
        } else {
            let a = self.offset(-1, 0).erosion(map);
            let b = self.offset(0, -1).erosion(map);
            a * b
        };
        let g = (g + map.depth) % 20183;
        map[self].erosion = Some(g);
        g
    }

    fn region(&self, map: &mut Map) -> Region {
        /* A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. */
        let erosion_level = self.erosion(map);
        let result = match erosion_level % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => panic!("math doesn't"),
        };
        result
    }

    fn can_use(&self, map: &mut Map, tool: &Tools) -> bool {
        if self.x < 0 || self.y < 0 || self.y as usize >= map.height || self.x as usize >= map.width {
            return false;
        }
        let region = self.region(map);
        match region {
            Region::Rocky => *tool == Tools::Climb || *tool == Tools::Torch,
            Region::Wet => *tool == Tools::Climb || *tool == Tools::Neither,
            Region::Narrow => *tool == Tools::Neither || *tool == Tools::Torch,
        }
    }

}

#[derive(Clone)]
struct Me {
    tool: Tools,
    pos: Pos,
    minutes: u64,
}

impl Me {
    fn move_to(&self, pos: Pos) -> Me {
        Me { pos, minutes: self.minutes+1, ..self.clone() }
    }
    fn use_tool(&self, tool: Tools) -> Me {
        Me { tool, minutes: self.minutes+7, ..self.clone() }
    }
}

fn main() {
    /* 
    depth: 4848
    target: 15,700
    */
    let target = Pos::new(15,700);
    let mut map = Map::new(&target, target.y as usize + 100, target.x as usize + 100);
    map.depth = 4848;
    let mut risk = 0;
    for y in 0..=target.y {
        for x in 0..=target.x {
            risk += match Pos::new(x, y).region(&mut map) {
                Region::Rocky => 0,
                Region::Wet => 1,
                Region::Narrow => 2,
            }
        }
    }
    println!("risk for rectangle: {}", risk);

    let mut heads = std::collections::VecDeque::new();
    let start = Me { pos: Pos::new(0, 0), tool: Tools::Torch, minutes: 0 };
    heads.push_back(start);

    while let Some(me) = heads.pop_front() {
        /*
            Assume me is in a valid configuration.  Might not be lowest cost.
        */
        let here = map[&me.pos].clone();
        if here.cost(&me.tool) <= me.minutes {
            // we've already been here, but at least as quickly.
            continue;
        }

        map[&me.pos].set_cost(&me.tool, me.minutes);
        // check movement
        for nearby in [me.pos.offset(1,0), me.pos.offset(0,1), me.pos.offset(-1,0), me.pos.offset(0,-1)].iter() {
            if nearby.can_use(&mut map, &me.tool) {
                let moved = me.move_to(nearby.clone());
                heads.push_back(moved);
            }
        }

        // check equiment change
        for new_tool in [Tools::Climb, Tools::Torch, Tools::Neither].iter() {
            if me.tool != *new_tool && me.pos.can_use(&mut map, new_tool) {
                let switched = me.use_tool(new_tool.clone());
                if switched.minutes < here.cost(new_tool) {
                    heads.push_back(switched);
                }
            }
        }
    }
    println!("Minutes to reach target holding torch: {:?}", map[&target].torch_cost);
}
