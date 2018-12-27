extern crate regex;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug,Clone)]
struct CPU {
    regs: Vec<i32>,
    ip_reg: i8,
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
        CPU { regs: vec!(0,0,0,0,0,0), ip_reg: 0 }
    }

    fn exec(&mut self, op: &Op, a: i32, b: i32, c: i32) -> Option<()> {
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

    fn run(&mut self, program: &Vec<Instr>) {
        while let Some(instr) = program.get(self.regs[self.ip_reg as usize] as usize) {
            self.exec(&instr.op, instr.a, instr.b, instr.c);
            self.regs[self.ip_reg as usize] += 1;
        }
    }
}

#[derive(Debug)]
struct Instr {
    op: Op,
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let input = include_str!("19_input");
    
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


    let mut cpu = CPU::new();

    let mut program = Vec::new();

    for line in input.lines() {
        let tokens : Vec<_> = line.split_whitespace().collect();
        if tokens[0] == "#ip" {
            cpu.ip_reg = tokens[1].parse().unwrap();
        } else {
            program.push(Instr {
                op: instr_map[tokens[0]].clone(),
                a: tokens[1].parse().unwrap(),
                b: tokens[2].parse().unwrap(),
                c: tokens[3].parse().unwrap(),
            });
        }
    }

    cpu.run(&program);
    println!("Part 1: {:?}", cpu);

    println!("Part 2 was completed by code inspection.  The code uses a triple nested loop to find the sum of the factors of 10551398.");

}