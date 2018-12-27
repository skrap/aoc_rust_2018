extern crate regex;

struct CPU {
    regs: Vec<i32>,
}

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
        
        match op {
            Op::Addr => *regs.get_mut(c_u)? = *regs.get(a_u)? + *regs.get(b_u)?,
            Op::Addi => *regs.get_mut(c_u)? = *regs.get(a_u)? + b,
            Op::Mulr => *regs.get_mut(c_u)? = *regs.get(a_u)? * *regs.get(b_u)?,
            Op::Muli => *regs.get_mut(c_u)? = *regs.get(a_u)? * b,
            Op::Banr => *regs.get_mut(c_u)? = *regs.get(a_u)? & *regs.get(b_u)?,
            Op::Bani => *regs.get_mut(c_u)? = *regs.get(a_u)? & b,
            Op::Borr => *regs.get_mut(c_u)? = *regs.get(a_u)? | *regs.get(b_u)?,
            Op::Bori => *regs.get_mut(c_u)? = *regs.get(a_u)? | b,
            Op::Setr => *regs.get_mut(c_u)? = *regs.get(a_u)?,
            Op::Seti => *regs.get_mut(c_u)? = a,
            Op::Gtir => *regs.get_mut(c_u)? = if a > *regs.get(b_u)? { 1 } else { 0 },
            Op::Gtri => *regs.get_mut(c_u)? = if *regs.get(a_u)? > b { 1 } else { 0 },
            Op::Gtrr => *regs.get_mut(c_u)? = if *regs.get(a_u)? > *regs.get(b_u)? { 1 } else { 0 },
            Op::Eqir => *regs.get_mut(c_u)? = if a == *regs.get(b_u)? { 1 } else { 0 },
            Op::Eqri => *regs.get_mut(c_u)? = if *regs.get(a_u)? == b { 1 } else { 0 },
            Op::Eqrr => *regs.get_mut(c_u)? = if *regs.get(a_u)? == *regs.get(b_u)? { 1 } else { 0 },
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
    let mut answer = 0;
    for caps in re.captures_iter(input) {
        let mut possible_ops = 0;
        let nums : Vec<i32> = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap()).collect();
        for op in all_ops.iter() {
            cpu.regs.clone_from_slice(&nums[0..4]);
            cpu.exec(op, nums[5], nums[6], nums[7]);
            if cpu.regs[..] == nums[8..] {
                possible_ops += 1;
            }
        }
        if possible_ops >=3 {
            println!("opcode {} matches {} ops", nums[4], possible_ops);
            answer += 1;
        }
    }
    println!("samples behaving like 3+ opcodes: {}", answer);
}