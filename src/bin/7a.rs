extern crate regex;
use std::collections::BTreeMap;

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

fn main() {
    let input = include_str!("7_input");
    
    let mut steps : BTreeMap<char, Step> = BTreeMap::new();

    // e.g. "Step A must be finished before step Q can begin."
    let re = regex::Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    for caps in re.captures_iter(input) {
        let dep = caps[1].chars().next().unwrap();
        let name = caps[2].chars().next().unwrap();

        steps.entry(name).or_insert(Step::new(name)).deps.push(dep);
        steps.entry(dep).or_insert(Step::new(dep));
    }

    let mut steps_log = String::new();

    while steps.len() > 0 {
        // find all steps with no deps
        let mut ready_names : Vec<_> = steps.iter()
            .filter(|(_k,s)| s.deps.len() == 0)
            .map(|(_k,s)| s.name)
            .collect();

        // find the first alphabetically
        ready_names.sort();
    
        // remove that step from the map
        let ready_step = steps.remove(&ready_names[0]).unwrap();
        println!("{:?} is ready", &ready_step);
        
        // remove that step from all deps lists
        let ready_name = &ready_step.name;
        for (_k,step) in steps.iter_mut() {
            let mut new_deps = step.deps.drain(..).filter(|d| d != ready_name).collect();
            step.deps = new_deps;
            // println!("{} new deps: {:?}", step.name, step.deps);
        }

        // add the step name to the steps log
        steps_log.push(ready_step.name);
    }

    println!("{}", steps_log);
}