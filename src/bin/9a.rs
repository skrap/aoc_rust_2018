extern crate linked_list;
use linked_list::{LinkedList, Cursor};

struct Board<'a> {
    current: Cursor<'a, i32>,
    free_marbles: std::ops::RangeInclusive<i32>,
}

impl<'a> Board<'a> {
    fn new(max_marble: i32, ring: &'a mut LinkedList<i32>) -> Board<'a> {
        ring.push_front(0);
        let current = ring.cursor();
        Board { current, free_marbles: 1..=max_marble }
    }

    fn forward(&mut self, count: usize) {
        let mut done = 0;
        while done < count {
            if let Some(_e) = self.current.next() {
                done += 1;
            }
        }
    }

    fn backward(&mut self, count: usize) {
        let mut done = 0;
        while done < count {
            if let Some(_e) = self.current.prev() {
                done += 1;
            }
        }
    }

    fn pop_lowest_marble(&mut self) -> Option<i32> {
        self.free_marbles.next()
    }

    fn play_rule_23(&mut self) -> i32 {
        self.backward(7);
        self.current.remove().unwrap()
    }

    fn play_normal(&mut self, id: i32) {
        self.forward(2);
        self.current.insert(id);
    }
}

struct Player {
    score: i64,
}

impl Player {
    fn new() -> Player {
        Player { score: 0 }
    }

    fn play(&mut self, board : &mut Board) -> bool {
        if let Some(id) = board.pop_lowest_marble() {
            if id % 23 == 0 {
                self.score += id as i64;
                self.score += board.play_rule_23() as i64;
            } else {
                board.play_normal(id);
            }
            true
        } else {
            false
        }
    }
}

fn main() {   
    // 471 players; last marble is worth 72026 points
    const NUM_PLAYERS : usize = 471;
    const MAX_MARBLE : i32 = 72026*100;
    // const NUM_PLAYERS : usize = 13;
    // const MAX_MARBLE : i32 = 7999;

    let mut ring: LinkedList<i32> = LinkedList::new();  // marble IDs in the ring.
    let mut board = Board::new(MAX_MARBLE, &mut ring);
    let mut players : Vec<_> = (0..NUM_PLAYERS).map(|_s| Player::new()).collect();
    
    'game: loop {
        for player in players.iter_mut() {
            let cont = player.play(&mut board);
            if ! cont {
                break 'game;
            }
        }
    }
    println!("max score is {:?}", players.iter().map(|p| p.score).max());
}