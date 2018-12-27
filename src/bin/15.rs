use std::collections::VecDeque;

enum Tile {
    Space,
    Wall,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn new(pos: (i32, i32)) -> Pos {
        Pos { x: pos.0, y: pos.1 }
    }

    fn offset(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

struct Board(Vec<Vec<Tile>>);

impl Board {
    fn print(&self, units: &Vec<Unit>) {
        let mut board_rows: Vec<Vec<u8>> = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Space => b'.',
                        Tile::Wall => b'#',
                    })
                    .collect()
            })
            .collect();
        for unit in units {
            if unit.alive() {
                board_rows[unit.pos.y as usize][unit.pos.x as usize] = match unit.team {
                    Team::Elf => b'E',
                    Team::Goblin => b'G',
                }
            }
        }
        for row in board_rows.drain(0..) {
            println!("{}", String::from_utf8(row).unwrap());
        }
    }
}

#[derive(PartialEq)]
enum Team {
    Elf,
    Goblin,
}

struct Unit {
    team: Team,
    pos: Pos,
    hp: i32,
    attack: i32,
}

impl Unit {
    fn new(team: Team, pos: Pos) -> Unit {
        Unit {
            team,
            pos,
            attack: 3,
            hp: 200,
        }
    }

    fn alive(&self) -> bool {
        self.hp > 0
    }
}

struct Router {
    distances: Vec<Vec<i16>>,
}

impl Router {
    // make up a distance finding algorithm.
    fn new(board: &Board, units: &Vec<Unit>, start_pos: &Pos) -> Router {
        let mut distances = Vec::new();
        for row in &board.0 {
            let mut base = Vec::new();
            for tile in row {
                base.push(match tile {
                    Tile::Space => i16::max_value(),
                    Tile::Wall => -1,
                });
            }
            distances.push(base);
        }

        for unit in units {
            if unit.alive() {
                // dead units can be disregarded
                distances[unit.pos.y as usize][unit.pos.x as usize] = -2;
            }
        }
        distances[start_pos.y as usize][start_pos.x as usize] = 0;

        let mut probes = VecDeque::new();
        probes.push_back((start_pos.offset(1, 0), 1));
        probes.push_back((start_pos.offset(-1, 0), 1));
        probes.push_back((start_pos.offset(0, 1), 1));
        probes.push_back((start_pos.offset(0, -1), 1));

        while probes.len() > 0 {
            let (pos, dist) = probes.pop_front().unwrap();
            let spot = &mut distances[pos.y as usize][pos.x as usize];
            if *spot > dist {
                *spot = dist;
                probes.push_back((pos.offset(1, 0), dist + 1));
                probes.push_back((pos.offset(-1, 0), dist + 1));
                probes.push_back((pos.offset(0, 1), dist + 1));
                probes.push_back((pos.offset(0, -1), dist + 1));
            }
        }

        Router { distances }
    }

    fn dist_to(&self, pos: &Pos) -> i16 {
        self.distances[pos.y as usize][pos.x as usize]
    }

    fn print(&self, start_pos: &Pos) {
        let mut board_rows: Vec<Vec<u8>> = self
            .distances
            .iter()
            .map(|row| {
                row.iter()
                    //.map(|t| if *t < 0 { b' ' } else { )
                    .map(|t| {
                        if *t == -1 {
                            b' '
                        } else if *t == -2 {
                            b'U'
                        } else if *t == i16::max_value() {
                            b'-'
                        } else {
                            b'0' + (*t % 10) as u8
                        }
                    })
                    .collect()
            })
            .collect();
        board_rows[start_pos.y as usize][start_pos.x as usize] = b'*';
        for row in board_rows.drain(0..) {
            println!("{}", String::from_utf8(row).unwrap());
        }
    }
}

fn read_input(input: &[u8]) -> (Board, Vec<Unit>) {
    let mut units = Vec::new();
    let mut board = Vec::new();
    board.push(Vec::new());
    let mut pos = (0, 0);

    for b in input {
        if *b == b'\n' {
            board.push(Vec::new());
            pos.0 = 0;
            pos.1 += 1;
        } else {
            board.last_mut().unwrap().push(match b {
                b'#' => Tile::Wall,
                b'.' => Tile::Space,
                b'G' => {
                    units.push(Unit::new(Team::Goblin, Pos::new(pos)));
                    Tile::Space
                }
                b'E' => {
                    units.push(Unit::new(Team::Elf, Pos::new(pos)));
                    Tile::Space
                }
                _ => panic!("Unknown tile: {} as pos {:?}", b, pos),
            });
            pos.0 += 1;
        }
    }

    (Board(board), units)
}

fn next_step(unit: &Unit, board: &Board, units: &Vec<Unit>) -> Option<Pos> {
    let targets: Vec<_> = units
        .iter()
        .filter(|u| u.team != unit.team && u.alive())
        .map(|u| {
            vec![
                u.pos.offset(0, 1),
                u.pos.offset(1, 0),
                u.pos.offset(0, -1),
                u.pos.offset(-1, 0),
            ]
        })
        .flatten()
        .collect();

    if targets.contains(&unit.pos) {
        return None;
    }

    let r = Router::new(&board, &units, &unit.pos);
    //r.print(&Pos::new((0,0)));

    let mut dist_targets: Vec<_> = targets
        .iter()
        .map(|t| (r.dist_to(t), t)) // (dist, pos)
        .filter(|c| c.0 < i16::max_value() && c.0 >= 0) // reachable
        .collect();
    dist_targets.sort_by(|a, b| a.0.cmp(&b.0)); // nearest
    if dist_targets.len() > 0 {
        let mut nearest: Vec<_> = dist_targets
            .iter()
            .filter(|dt| dt.0 == dist_targets[0].0)
            .map(|dt| dt.1)
            .collect();
        nearest.sort();
        let chosen = nearest[0];
        let chosen_router = Router::new(&board, &units, &chosen);
        let steps = [
            unit.pos.offset(1, 0),
            unit.pos.offset(-1, 0),
            unit.pos.offset(0, 1),
            unit.pos.offset(0, -1),
        ];
        let mut dist_steps: Vec<_> = steps
            .iter()
            .map(|p| (chosen_router.dist_to(p), p))
            .filter(|dt| dt.0 >= 0)
            .collect();
        dist_steps.sort();
        return Some(dist_steps[0].1.clone());
    }
    None
}

fn get_target(unit: &Unit, units: &Vec<Unit>) -> Option<usize> {
    let adjacent = [
        unit.pos.offset(1, 0),
        unit.pos.offset(-1, 0),
        unit.pos.offset(0, 1),
        unit.pos.offset(0, -1),
    ];
    let mut hp_pos_targets : Vec<_> = units.iter().enumerate().filter(|(_i,u)| u.alive() && u.team != unit.team && adjacent.contains(&u.pos)).collect();
    hp_pos_targets.sort_by(|(_ia,a),(_ib,b)| a.hp.cmp(&b.hp).then(a.pos.cmp(&b.pos)));
    if let Some((idx,_unit)) = hp_pos_targets.get(0) {
        return Some(*idx);
    }
    None
}

fn main() {
    let input = include_bytes!("15_input");
    //let input = include_bytes!("15_tiny");
    let (board, mut units) = read_input(input);
    board.print(&units);

    let mut round_count = 0;
    'game: loop {
        units.sort_by(|a,b| a.pos.cmp(&b.pos));
        for turn_idx in 0..units.len() {
            if ! units[turn_idx].alive() {
                continue;
            }
            // check for game over
            if units.iter().filter(|u| u.alive() && u.team != units[turn_idx].team).count() == 0 {
                break 'game;
            }

            if let Some(step) = next_step(&units[turn_idx], &board, &units) {
                units[turn_idx].pos = step;
            }
            if let Some(target_idx) = get_target(&units[turn_idx], &units) {
                units[target_idx].hp -= units[turn_idx].attack;
            }
        }
        for unit in &units {
            if unit.alive() {
                println!("{}({})", if unit.team == Team::Elf { 'E' } else { 'G' }, unit.hp);
            }
        }
        board.print(&units);
        round_count += 1;
    }

    let living : Vec<_> = units.iter().filter(|u| u.alive()).collect();
    let mut hp = 0;
    for unit in living {
        println!("Unit hp {}", unit.hp);
        hp += unit.hp;
    }
    println!("rounds: {},  hp remaining: {}", round_count, hp);
    println!("final answer: {}", round_count * hp);
}
