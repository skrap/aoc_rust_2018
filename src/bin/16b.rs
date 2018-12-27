extern crate regex;
use std::collections::BTreeMap;

struct CPU {
    regs: Vec<i32>,
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
        CPU { regs: vec!(0,0,0,0) }
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
}

fn main() {
    let input = include_str!("16_input");
    let all_ops = [Op::Addr,
                Op::Addi,
                Op::Mulr,
                Op::Muli,
                Op::Banr,
                Op::Bani,
                Op::Borr,
                Op::Bori,
                Op::Setr,
                Op::Seti,
                Op::Gtir,
                Op::Gtri,
                Op::Gtrr,
                Op::Eqir,
                Op::Eqri,
                Op::Eqrr];
    
    let re = regex::Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let mut cpu = CPU::new();


    let mut ops : Vec<Vec<Option<bool>>> = Vec::new(); // ops[opcode][..] are the possible all_ops indexes for this opcode
    ops.resize(all_ops.len(), Vec::new());
    for subvec in ops.iter_mut() {
        subvec.resize(all_ops.len(), None);
    }

    for caps in re.captures_iter(input) {
        let nums : Vec<i32> = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap()).collect();
        for (op_idx,op) in all_ops.iter().enumerate() {
            cpu.regs.clone_from_slice(&nums[0..4]);
            let success = cpu.exec(op, nums[5], nums[6], nums[7]).is_some() && cpu.regs[..] == nums[8..];
            ops[nums[4] as usize][op_idx] = Some(success && ops[nums[4] as usize][op_idx].unwrap_or(true));
        }
    }

    let print_ops = |ops: &Vec<Vec<Option<bool>>>| {
        print!("    ");
        for op in all_ops.iter() {
            print!(" {:?}", op);
        }
        print!("\n");
        for (opcode, op_tests) in ops.iter().enumerate() {
            println!("{:02}: {}", opcode, op_tests.iter().map(|b| match *b {
                None => "|    ",
                Some(true) => "|  Y ",
                Some(false) => "|  . ",    
            }).collect::<String>());
        }
    };

    loop {
        let mut unsolved = false;
        for opcode in 0..ops.len() {
            if ops[opcode].iter().filter(|b| b.unwrap()).count() == 1 {
                let op_idx = ops[opcode].iter().position(|b| b.unwrap()).unwrap();
                for other_opcode in 0..ops.len() {
                    if other_opcode != opcode {
                        ops[other_opcode][op_idx] = Some(false);
                    }
                }
            } else {
                unsolved = true;
            }
        }
        if ! unsolved {
            break;
        }
    }
    print_ops(&ops);

    let mut opcode_to_op : BTreeMap<usize,Op> = BTreeMap::new();
    for (opcode, op_indexes) in ops.iter().enumerate() {
        let pos = op_indexes.iter().position(|b| b.unwrap()).unwrap();
        opcode_to_op.insert(opcode, all_ops[pos].clone());
    }

    // check it
    for caps in re.captures_iter(input) {
        let nums : Vec<i32> = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap()).collect();
        cpu.regs.clone_from_slice(&nums[0..4]);
        let op = opcode_to_op.get(&(nums[4] as usize)).unwrap();
        let success = cpu.exec(op, nums[5], nums[6], nums[7]).is_some() && cpu.regs[..] == nums[8..];
        assert!(success);
    }


    let program = include_str!("16b_input");
    let prog_re = regex::Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let mut prog_cpu = CPU::new();
    for caps in prog_re.captures_iter(program) {
        let nums : Vec<i32> = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap()).collect();
        let op = opcode_to_op.get(&(nums[0] as usize)).unwrap();
        if prog_cpu.exec(op, nums[1], nums[2], nums[3]).is_none() {
            panic!("couldn't execute {:?}", (op, nums[1], nums[2], nums[3]));
        }
    }
    println!("CPU Regs after program: {:?}", &prog_cpu.regs);
}