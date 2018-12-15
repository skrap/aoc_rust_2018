#[derive(Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self, turn : &NextTurn) -> Dir {
        match turn {
            NextTurn::Left => {
                match self {
                    Dir::Up => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Right => Dir::Up,
                }
            }
            NextTurn::Right => {
                match self {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                }
            } 
            _ => {
                self.clone()
            }
        }
    }
}

#[derive(Clone)]
enum NextTurn {
    Left,
    Straight,
    Right,
}

#[derive(Clone)]
struct Cart {
    pos: (i32, i32),
    dir: Dir,
    next_turn: NextTurn,
}

impl Cart {
    fn new(pos: (i32, i32), dir: Dir) -> Cart {
        Cart {pos, dir, next_turn: NextTurn::Left}
    }

    fn play(&mut self, board: &Board) {
        match self.dir {
            Dir::Left => self.pos.0 -= 1,
            Dir::Right => self.pos.0 += 1,
            Dir::Up => self.pos.1 -= 1,
            Dir::Down => self.pos.1 += 1,
        }
        let track = &board[self.pos.1 as usize][self.pos.0 as usize];
        match track {
            Track::TurnUpL => {
                self.dir = match self.dir {
                     Dir::Up => Dir::Left,
                     Dir::Right => Dir::Down,
                     Dir::Left => Dir::Up,
                     Dir::Down => Dir::Right,
                }
            },
            Track::TurnUpR => {
                self.dir = match self.dir {
                    Dir::Up => Dir::Right,
                    Dir::Left => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Right => Dir::Up,
                }
            },
            Track::Cross => {
                self.dir = self.dir.turn(&self.next_turn);
                self.next_turn = match self.next_turn {
                    NextTurn::Left => { NextTurn::Straight },
                    NextTurn::Straight => { NextTurn::Right },
                    NextTurn::Right => { NextTurn::Left },
                };
            }
            Track::Empty => panic!("Cart derailed onto {:?}", self.pos),
            _ => ()
        }
    }
}

enum Track {
    Empty,   // ' '
    Horiz,   //  -
    Vert,    //  |
    TurnUpL, //  \
    TurnUpR, //  /
    Cross,   //  +
}

type Board = Vec<Vec<Track>>;

fn main() {
    let input = include_bytes!("13_input");
    let mut board : Board = Vec::new();
    board.push(Vec::new());
    let mut carts : Vec<Cart> = Vec::new();

    let mut pos = (0,0);
    for b in input.iter() {
        match b {
            b'\n' => {
                board.push(Vec::new());
                pos.1 += 1;
                pos.0 = 0;
            },
            _ => {
                board.last_mut().unwrap().push(match b {
                    b' ' => Track::Empty,
                    b'-' => Track::Horiz,
                    b'|' => Track::Vert,
                    b'/' => Track::TurnUpR,
                    b'\\'=> Track::TurnUpL,
                    b'+' => Track::Cross,
                    b'^' => {
                        carts.push(Cart::new(pos, Dir::Up));
                        Track::Vert
                    },
                    b'>' => {
                        carts.push(Cart::new(pos, Dir::Right));
                        Track::Horiz
                    },
                    b'v' => {
                        carts.push(Cart::new(pos, Dir::Down));
                        Track::Vert
                    },
                    b'<' => {
                        carts.push(Cart::new(pos, Dir::Left));
                        Track::Horiz
                    }
                    _    => panic!("unknown byte {}", b),
                });
                pos.0 += 1;
            }
        }
    }

    while carts.len() > 1 {
        carts.sort_by(|a,b| a.pos.1.cmp(&b.pos.1).then(a.pos.0.cmp(&b.pos.0)).reverse());
        let mut next_carts : Vec<Cart> = Vec::new();
        while carts.len() > 0 {
            let mut cart = carts.pop().unwrap();
            cart.play(&mut board);
            if let Some(idx) = carts.iter().position(|c| c.pos == cart.pos) {
                println!("collision at {:?}", cart.pos);
                carts.remove(idx);
            } else if let Some(idx) = next_carts.iter().position(|c| c.pos == cart.pos) {
                println!("collinion at {:?}", cart.pos);
                next_carts.remove(idx);
            } else {
                next_carts.push(cart);
            }
        }
        carts = next_carts;
    }
    println!("position of remaining cart: {:?}", carts[0].pos);
}