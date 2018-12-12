extern crate regex;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(Debug)]
struct Step {
    deps : Vec<char>,
    name : char,
}

impl Step {
    fn new(name: char) -> Step {
        Step {name, deps: Vec::new()}
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq)]
struct Event {
    priority: i32,
    time: i32,
    step: char,
}

struct Scheduler {
    steps : BTreeMap<char, Step>,
    events: BinaryHeap<Event>,
    time: i32,
    free_workers: i32,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler { steps: BTreeMap::new(), events: BinaryHeap::new(), time: 0, free_workers: 5 }
    }

    fn add_dep(&mut self, name: char, dep: char) {
        self.steps.entry(name).or_insert(Step::new(name)).deps.push(dep);
        self.steps.entry(dep).or_insert(Step::new(dep));
    }

    fn pop_ready_step(&mut self) -> Option<char> {
        // find all steps with no deps
        let mut ready_names : Vec<_> = self.steps.iter()
            .filter(|(_k,s)| s.deps.len() == 0)
            .map(|(_k,s)| s.name)
            .collect();

        // find the first alphabetically
        ready_names.sort();

        if let Some(&c) = ready_names.get(0) {
            self.steps.remove(&c);
            Some(c)
        } else {
            None
        }
    }

    fn resolve_step(&mut self, name: char) {
        println!("{} is ready", &name);
        
        // remove that step from all deps lists
        for (_k,step) in self.steps.iter_mut() {
            let mut new_deps = step.deps.drain(..).filter(|d| d != &name).collect();
            step.deps = new_deps;
            // println!("{} new deps: {:?}", step.name, step.deps);
        }
    }

    fn step_delay(name: char) -> i32 {
        61 + (name as i32 - 'A' as i32)
    }

    fn tick(&mut self) -> bool {
        println!("time: {}", self.time);
        // eval each ready event
        loop {
            { 
                let event = self.events.peek();
                if event.is_none()|| event.unwrap().time != self.time {
                    break;
                }
            }
            let event = self.events.pop().unwrap();
            self.resolve_step(event.step);
            self.free_workers += 1
        }

        while self.free_workers > 0 {
            if let Some(name) = self.pop_ready_step() {
                println!("starting job {}", name);
                let job_done = self.time + Self::step_delay(name);
                self.events.push(Event { step: name, time: job_done, priority: -job_done });
                self.free_workers -= 1;
            } else {
                break;
            }
        }

        if let Some(event) = self.events.peek() {
            self.time = event.time;
            return true;
        } else {
            return false; // no events left.
        }
    }

    fn get_time(&self) -> i32 {
        self.time
    }
}

fn main() {
    let input = include_str!("7_input");

    let mut sched = Scheduler::new();    

    // e.g. "Step A must be finished before step Q can begin."
    let re = regex::Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    for caps in re.captures_iter(input) {
        let dep = caps[1].chars().next().unwrap();
        let name = caps[2].chars().next().unwrap();
        sched.add_dep(name, dep);
    }

    for (k, val) in sched.steps.iter() {
        println!("{}: {:?}", k, val);
    }

    while sched.tick() {
        println!("time: {}, {} steps incomplete", sched.get_time(), sched.events.len() + sched.steps.len());
    }
    println!("done at time {}", sched.get_time());
}