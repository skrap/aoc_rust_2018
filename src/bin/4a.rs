extern crate regex;
use regex::Regex;
use std::collections::BTreeMap;

struct GuardRecord {
    id : u32,
    sleepy_time : [u32; 100],
}

impl GuardRecord {
    pub fn new(id : u32) -> GuardRecord {
        GuardRecord {id, sleepy_time: [0u32; 100]}
    }

    pub fn add_sleep(&mut self, start_min: u32, end_min: u32) {
        for i in start_min..end_min {
            self.sleepy_time[i as usize] += 1;
        }
    }

    pub fn most_sleepy_min(&self) -> usize {
        let mut max_i = 0usize;
        let mut max_val = 0u32;
        for (i, &val) in self.sleepy_time.iter().enumerate() {
            if max_val < val {
                max_i = i;
                max_val = val;
            }
        }
        return max_i;
    }

    pub fn mins_sleeping(&self) -> u32 {
        self.sleepy_time.iter().sum()
    }
}

fn main() {
    let input = include_str!("4a_input");
    let mut lines : Vec<_> = input.lines().collect();
    lines.sort_unstable();

    let mut records : BTreeMap<u32,GuardRecord> = BTreeMap::new();

    // [1518-09-22 23:50] Guard #2309 begins shift
    // [1518-06-26 00:42] falls asleep
    // [1518-10-09 00:34] wakes up
    let re = Regex::new(r"Guard #(\d+) begins|:(\d\d)\] falls|:(\d\d)\] wakes").unwrap();
    
    let mut cur_guard_num = None;
    let mut sleeps_min = None;
    for line in lines {
        let caps = re.captures(line).unwrap();
        if caps.get(1).is_some() {
            let guard_num : u32 = caps[1].parse().unwrap();
            if ! records.contains_key(&guard_num) {
                records.insert(guard_num, GuardRecord::new(guard_num));
            }
            cur_guard_num = Some(guard_num);
            sleeps_min = None;
        } else if caps.get(2).is_some() {
            sleeps_min = Some(caps[2].parse().unwrap());
        } else if caps.get(3).is_some() {
            let wakes_min = Some(caps[3].parse().unwrap());
            records.get_mut(&cur_guard_num.unwrap()).unwrap().add_sleep(sleeps_min.unwrap(), wakes_min.unwrap());
            sleeps_min = None;
        }
    }

    let mut sleepy_sorted : Vec<_> = records.values().collect();
    sleepy_sorted.sort_unstable_by(|a,b| a.mins_sleeping().cmp(&b.mins_sleeping()));
    let sleepiest = &sleepy_sorted[sleepy_sorted.len()-1];
    println!("most sleepiest guard: {}.  Likeliest nap: {} mins", sleepiest.id, sleepiest.most_sleepy_min());
    println!("puzzle answer (part 1): {}", sleepiest.id * sleepiest.most_sleepy_min() as u32);

    let mut sleepiest_min_sorted : Vec<_> = records.values().collect();
    sleepiest_min_sorted.sort_unstable_by(|a, b| {
        a.sleepy_time[a.most_sleepy_min()].cmp(&b.sleepy_time[b.most_sleepy_min()])
    });
    let sleepiest_by_min = &sleepiest_min_sorted[sleepiest_min_sorted.len()-1];
    println!("guard {} slept {} times on min {}", 
             sleepiest_by_min.id, 
             sleepiest_by_min.sleepy_time[sleepiest_by_min.most_sleepy_min()],
             sleepiest_by_min.most_sleepy_min()
             );
    println!("puzzle part 2 answer: {}", sleepiest_by_min.id * sleepiest_by_min.most_sleepy_min() as u32);
}