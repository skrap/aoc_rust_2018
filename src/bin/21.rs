
extern crate regex;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug,Clone)]
struct CPU {
    regs: Vec<i64>,
    ip_reg: i8,
    instr_count: i64,
}

#[derive(Debug,Clone)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl CPU {
    fn new() -> CPU {
        CPU { regs: vec!(0,0,0,0,0,0), ip_reg: 0, instr_count: 0 }
    }

    fn exec(&mut self, op: &Op, a: i64, b: i64, c: i64) -> Option<()> {
        let regs = &mut self.regs;
        let a_u = a as usize;
        let b_u = b as usize;
        let c_u = c as usize;
        
        *regs.get_mut(c_u)? = match op {
            Op::Addr => *regs.get(a_u)? + *regs.get(b_u)?,
            Op::Addi => *regs.get(a_u)? + b,
            Op::Mulr => *regs.get(a_u)? * *regs.get(b_u)?,
            Op::Muli => *regs.get(a_u)? * b,
            Op::Banr => *regs.get(a_u)? & *regs.get(b_u)?,
            Op::Bani => *regs.get(a_u)? & b,
            Op::Borr => *regs.get(a_u)? | *regs.get(b_u)?,
            Op::Bori => *regs.get(a_u)? | b,
            Op::Setr => *regs.get(a_u)?,
            Op::Seti => a,
            Op::Gtir => if a > *regs.get(b_u)? { 1 } else { 0 },
            Op::Gtri => if *regs.get(a_u)? > b { 1 } else { 0 },
            Op::Gtrr => if *regs.get(a_u)? > *regs.get(b_u)? { 1 } else { 0 },
            Op::Eqir => if a == *regs.get(b_u)? { 1 } else { 0 },
            Op::Eqri => if *regs.get(a_u)? == b { 1 } else { 0 },
            Op::Eqrr => if *regs.get(a_u)? == *regs.get(b_u)? { 1 } else { 0 },
        };
        Some(())
    }

    fn run(&mut self, program: &Vec<Instr>) -> (i64,i64) {
        let mut states = BTreeSet::new();
        let mut answer = (0,0);
        loop {
            let idx = self.regs[self.ip_reg as usize] as usize;
            if let Some(instr) = program.get(idx) {
                if idx == 17 {
                    // this loop takes forever.  hotpatch it, cuz I have the power of DIVISION.
                    self.regs[3] = self.regs[4]/256;
                    self.instr_count += 7*self.regs[3];
                } else {
                    if idx == 28 {
                        if states.contains(&self.regs) {
                            println!("state {:?} repeated.  Exiting.", &self.regs);
                            break;
                        } else {
                            println!("state {:?} at {}", &self.regs, self.instr_count);
                            if answer.0 == 0 {
                                answer.0 = self.regs[5];
                            } else {
                                answer.1 = self.regs[5];
                            }
                            states.insert(self.regs.clone());
                        }
                    }
                    self.exec(&instr.op, instr.a, instr.b, instr.c);
                }
                self.regs[self.ip_reg as usize] += 1;
                self.instr_count += 1;
                //println!("{}: {:?} => {:?}  ic: {}", idx, instr, self.regs, self.instr_count);
            } else {
                break;
            }
        }
        return answer;
    }

    fn reset(&mut self) {
        self.instr_count = 0;
        self.regs.copy_from_slice(&[0;6]);
    }

    fn load(&mut self, input: &str) -> Vec<Instr> {
        let mut instr_map : BTreeMap<&str, Op> = BTreeMap::new();
        instr_map.insert("addr", Op::Addr);
        instr_map.insert("addi", Op::Addi);
        instr_map.insert("mulr", Op::Mulr);
        instr_map.insert("muli", Op::Muli);
        instr_map.insert("banr", Op::Banr);
        instr_map.insert("bani", Op::Bani);
        instr_map.insert("borr", Op::Borr);
        instr_map.insert("bori", Op::Bori);
        instr_map.insert("setr", Op::Setr);
        instr_map.insert("seti", Op::Seti);
        instr_map.insert("gtir", Op::Gtir);
        instr_map.insert("gtri", Op::Gtri);
        instr_map.insert("gtrr", Op::Gtrr);
        instr_map.insert("eqir", Op::Eqir);
        instr_map.insert("eqri", Op::Eqri);
        instr_map.insert("eqrr", Op::Eqrr);

        let mut program = Vec::new();

        for line in input.lines() {
            let tokens : Vec<_> = line.split_whitespace().collect();
            if tokens[0] == "#ip" {
                self.ip_reg = tokens[1].parse().unwrap();
            } else {
                program.push(Instr {
                    op: instr_map[tokens[0]].clone(),
                    a: tokens[1].parse().unwrap(),
                    b: tokens[2].parse().unwrap(),
                    c: tokens[3].parse().unwrap(),
                });
            }
        }

        program
    }
}

#[derive(Debug,Clone)]
struct Instr {
    op: Op,
    a: i64,
    b: i64,
    c: i64,
}

fn main() {
    let input = include_str!("21_input");
    let mut cpu = CPU::new();
    let program = cpu.load(input);

    cpu.reset();
    let answers = cpu.run(&program);
    println!("finished in {} instrs", cpu.instr_count);

    println!("answer to part a: {}", answers.0);
    println!("answer to part b: {}", answers.1);
}    
