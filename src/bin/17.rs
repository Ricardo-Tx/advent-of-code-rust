use std::{collections::{HashSet, VecDeque}, io::Cursor};

advent_of_code::solution!(17);

fn compute(reg: &mut [u64;3], prog: &Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut cursor : usize = 0;

    let make_combo = |op: &mut u64, r: &[u64;3]| { 
        if *op > 3 && *op < 7 {
            *op = r[(*op-4) as usize];
        }
    };
    
    loop {
        if cursor >= prog.len() {
            break;  // halt
        }

        let mut jump : bool = true;
        let mut op = prog[cursor+1] as u64;

        match prog[cursor] {
            0 => {  // adv
                make_combo(&mut op, &reg);
                reg[0] >>= op;
            }, 
            1 => {  // bxl
                reg[1] ^= op; 
            },
            2 => {  // bst
                make_combo(&mut op, &reg);
                reg[1] = op & 7;
            },
            3 => {  // jnz
                if reg[0] != 0 {
                    cursor = op as usize;
                    jump = false;
                }
            }, 
            4 => {  // bxc
                reg[1] = reg[1] ^ reg[2]; 
            }, 
            5 => {  // out    
                make_combo(&mut op, &reg);
                out.push((op & 7) as u8); 
            }, 
            6 => {  // bdv
                make_combo(&mut op, &reg);
                reg[1] = reg[0] >> op;
            }, 
            7 => {  // cdv
                make_combo(&mut op, &reg);
                reg[2] = reg[0] >> op;
            }, 
            _ => {},
        }

        if jump {
            cursor += 2;
        }
    }

    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.split('\n');

    let mut reg : [u64;3] = [0, 0, 0];
    for i in 0..3 {
        let mut it_r = it.next()?.split(": ");
        it_r.next();
        reg[i] = it_r.next()?.parse().expect("valid!");
    }

    it.next();
    let mut it_p = it.next()?.split(": ");
    it_p.next();
    let mut program : Vec<u8> = Vec::new();
    for b in it_p.next()?.split(',') {
        program.push(b.parse().expect("valid!"));
    };
    
    let out: Vec<u8> = compute(&mut reg, &program);
    let mut res : u64 = 0;
    for o in out.iter() {
        res *= 10;
        res += *o as u64;
    }
    println!("{:?}", out);
    Some(res)
}

// 2,4,1,2,7,5,4,3,0,3,1,7,5,5,3,0
//
// 2,4 -> reg[1] = reg[0] & 7;
// 1,2 -> reg[1] ^= 2;
// 7,5 -> reg[2] = reg[0] >> reg[1];
// 4,3 -> reg[1] ^= reg[2];
// 0,3 -> reg[0] >>= 3;
// 1,7 -> reg[1] ^= 7;
// 5,5 -> out(reg[1] & 7);
// 3,0 -> if A cursor(0);

pub fn part_two(input: &str) -> Option<u64> {
    let mut it = input.split('\n');

    let mut reg : [u64;3] = [0, 0, 0];
    for i in 0..3 {
        let mut it_r = it.next()?.split(": ");
        it_r.next();
        reg[i] = it_r.next()?.parse().expect("valid!");
    }

    it.next();
    let mut it_p = it.next()?.split(": ");
    it_p.next();
    let mut program : Vec<u8> = Vec::new();
    let mut shift_amount : u8 = 0;
    for (i, b) in it_p.next()?.split(',').enumerate() {
        let n : u8 = b.parse().expect("valid!");
        program.push(n);
        if shift_amount == u8::MAX {
            shift_amount = n;
        } else if n == 0 && i%2 == 0 {
            shift_amount = u8::MAX;
        }
    };
    let program_len = program.len();
    println!("Program: {:?}", program);


    // the program only halts if A = 0 and only opcode 0 can write to A so
    // we can get a low estimate to start the calculations
    // let mut val : u64 = 1 << (shift_amount * program_len as u8 - 3);
    let mut cur_shift : usize = 1;
    let mut vals : VecDeque<u64> = VecDeque::from_iter(1..1<<shift_amount);
    let mut first : u64 = 0;
    while !vals.is_empty() {
        let base = vals.pop_front()?;
        if base >> (cur_shift as u8 * shift_amount) > 0 {
            cur_shift += 1;
        }

        let out = compute(&mut [base, 0, 0], &program);
        print!("base: {}, out: {:?}, cur_shift: {}", base, out, cur_shift);


        if out[0] == program[program_len-cur_shift] {
            if cur_shift == program_len {
                println!(" final");
                first = base;
                break;
            }
            // tentatively
            for i in 0u64..1<<shift_amount {
                vals.push_back((base << shift_amount) | i);
            }
            // println!("{base} OK, ");
            println!(" pass");
        } else {
            println!(" fail");
        }
    }

    // println!("{:?}", out);
    Some(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
