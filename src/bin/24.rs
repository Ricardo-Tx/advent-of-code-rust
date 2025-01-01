use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(24);

#[derive(Eq, Hash, PartialEq, Debug)]
enum Operation {
    Or,
    And,
    Xor,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Gate<'a> {
    operation: Operation,
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn logical_output(&self, map: &HashMap<&str, String>) -> String {
        if !map.contains_key(self.input1) {
            return "no_key_1".to_owned();
        } else if !map.contains_key(self.input2) {
            return "no_key_2".to_owned();
        }
        if self.output.bytes().next() == Some(b'z') {
            return self.output.to_string();
        }

        let mut t1 : u8 = match map[self.input1].bytes().next() {
            Some(x) => x,
            None => return String::new(),
        };
        let mut t2 : u8 = match map[self.input2].bytes().next() {
            Some(x) => x,
            None => return String::new(),
        };
        let mut num1 : u8 = match map[self.input1][1..].parse::<u8>() {
            Ok(x) => x,
            Err(_) => return String::new(),
        
        };
        let mut num2 : u8 = match map[self.input2][1..].parse::<u8>() {
            Ok(x) => x,
            Err(_) => return String::new(),
        };

        if t1 > t2 {
            let temp = t1;
            t1 = t2;
            t2 = temp;
            let temp = num1;
            num1 = num2;
            num2 = temp;
        }

        if t1 == b'x' && t2 == b'y' && num1 == num2 {
            if num2 == 0 {
                // half adder
                match self.operation {
                    Operation::Xor => return format!("z{num2:0>2}"),
                    Operation::And => return format!("c{num2:0>2}"),
                    _ => {},
                };
            } else {
                // full adder
                match self.operation {
                    Operation::Xor => return format!("i{num2:0>2}"),
                    Operation::And => return format!("m{num2:0>2}"),
                    _ => {},
                };
            }
        } else if t1 == b'c' && t2 == b'i' && num1+1 == num2 {
            match self.operation {
                Operation::Xor => return format!("z{num2:0>2}"),
                Operation::And => return format!("n{num2:0>2}"),
                _ => {},
            };
        } else if t1 == b'm' && t2 == b'n' && num1 == num2 && self.operation == Operation::Or {
            return format!("c{num2:0>2}");
        }

        String::new()
    }
}



fn simulate<'b>(gates: &'b Vec<Gate<'b>>, gate_deps: &HashMap<&'b str,HashSet<usize>>, wires: &mut HashMap<&str,Option<bool>>, queue: &mut VecDeque<(&'b str,bool)>) {
    while !queue.is_empty() {
        let next = match queue.pop_front() {
            Some(x) => x,
            None => continue,
        };
        if let Some(w) = wires.get_mut(next.0) {
            *w = Some(next.1);
        }
        // println!("{:?}", next);
        if let Some(set) = gate_deps.get(next.0) {
            for &i in set.iter() {
                // println!(" - {:?} depends on {:?}", gates[i], next.0);
                if let Some(a) = wires[gates[i].input1] {
                    if let Some(b) = wires[gates[i].input2] {
                        queue.push_back((gates[i].output, match gates[i].operation {
                            Operation::And => b && a,
                            Operation::Or => b || a,
                            Operation::Xor => b ^ a,
                        }));
                    }
                }
            }
        }
    } 
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut wires : HashMap<&str,Option<bool>> = HashMap::new();
    let mut gates : Vec<Gate> = Vec::new();
    let mut gate_dependencies : HashMap<&str,HashSet<usize>> = HashMap::new();

    let mut queue : VecDeque<(&str,bool)> = VecDeque::new();
    
    for line in input.split('\n').rev() {
        if line.is_empty() {
            continue;
        }
        
        if line.contains("->") {
            let mut pieces = line.split(' ');

            let input1 = pieces.next()?;
            let operation = match pieces.next()? {
                "OR" => Operation::Or,
                "AND" => Operation::And,
                "XOR" => Operation::Xor,
                _ => continue,
            };
            let input2 = pieces.next()?;
            pieces.next();
            let output = pieces.next()?;
            if !wires.contains_key(&input1) {
                wires.insert(input1, None);
            }
            if !wires.contains_key(&input2) {
                wires.insert(input2, None);
            }
            if !wires.contains_key(&output) {
                wires.insert(output, None);
            }

            let gate = Gate {
                operation,
                input1,
                input2,
                output,
            };
            gates.push(gate);
            let g_len = gates.len();

            if let Some(dep1) = gate_dependencies.get_mut(&input1) {
                (*dep1).insert(g_len-1);
            } else {
                gate_dependencies.insert(input1, HashSet::from([g_len-1]));
            }
            if let Some(dep2) = gate_dependencies.get_mut(&input2) {
                (*dep2).insert(g_len-1);
            } else {
                gate_dependencies.insert(input2, HashSet::from([g_len-1]));
            }
        } else {
            queue.push_back((&line[0..3], line.bytes().nth(5)? == b'1'));
        }
    }

    simulate(&gates, &gate_dependencies, &mut wires, &mut queue);

    let mut result : u64 = 0;
    for (&k, v) in wires.iter() {
        if k.bytes().next()? != b'z' {
            continue;
        }
        let n : u64 = k[1..].parse().expect("yahhh");
        if (*v)? {
            result |= 1 << n;
        }
    }

    Some(result)
}

// x12       AND y12       -> z12       (should be m12)
// i12 (ggr) XOR c11 (hnd) -> m12 (kwb) (should be z12)

// c15 (gkw) AND i16 (cmc) -> z16       (should be n16)
// c15 (gkw) XOR i16 (cmc) -> n16 (qkf) (should be z16)

// n24 (vhm) OR  m24 (wwd) -> z24       (should be c24)
// i24 (stj) XOR c23 (ttg) -> c24 (tgr) (should be z24)

// x29       XOR y29       -> cph       (should be jqn)
// x29       AND y29       -> jqn       (should be cph)

// cph,jqn,kwb,qkf,tgr,z12,z16,z24


// // n29 (vhm) OR m29 (wwd) -> c29
// c28 (rwq OK) AND i29 (cph XXX) -> n29


pub fn part_two(input: &str) -> Option<u64> {
    let mut renamed : HashMap<&str,bool> = HashMap::new();
    let mut gates : Vec<Gate> = Vec::new();
    let mut gate_dependencies : HashMap<&str,HashSet<usize>> = HashMap::new();

    let mut queue : VecDeque<&str> = VecDeque::new();
    
    for line in input.split('\n').rev() {
        if line.is_empty() {
            continue;
        }
        
        if line.contains("->") {
            let mut pieces = line.split(' ');

            let input1 = pieces.next()?;
            let operation = match pieces.next()? {
                "OR" => Operation::Or,
                "AND" => Operation::And,
                "XOR" => Operation::Xor,
                _ => continue,
            };
            let input2 = pieces.next()?;

            pieces.next();
            let output = pieces.next()?;
            if !renamed.contains_key(&input1) {
                renamed.insert(input1, false);
            }
            if !renamed.contains_key(&input2) {
                renamed.insert(input2, false);
            }
            if !renamed.contains_key(&output) {
                renamed.insert(output, false);
            }

            let gate = Gate {
                operation,
                input1: input1.min(input2),
                input2: input1.max(input2),
                output,
            };
            gates.push(gate);
            let g_len = gates.len();

            if let Some(dep1) = gate_dependencies.get_mut(&input1) {
                (*dep1).insert(g_len-1);
            } else {
                gate_dependencies.insert(input1, HashSet::from([g_len-1]));
            }
            if let Some(dep2) = gate_dependencies.get_mut(&input2) {
                (*dep2).insert(g_len-1);
            } else {
                gate_dependencies.insert(input2, HashSet::from([g_len-1]));
            }
        } else {
            queue.push_back(&line[0..3]);
        }
    }

    // renaming
    // let mut x_wires : Vec<&str> = Vec::new();
    // let mut y_wires : Vec<&str> = Vec::new();
    // let mut z_wires : Vec<&str> = Vec::new();
    // for &k in wires.keys() {
    //     let b = k.bytes().next()?;
    //     if b != b'x' && b != b'y' && b != b'z' {
    //         continue;
    //     }
    //     let vct = match b {
    //         b'x' => &mut x_wires,
    //         b'y' => &mut y_wires,
    //         _ => &mut z_wires,
    //     };
    //     let index = match k[1..].parse::<usize>() {
    //         Ok(x) => x,
    //         Err(_) => continue,
    //     };

    //     while index >= vct.len() {
    //         vct.push("___");
    //     }

    //     vct[index] = k;
    // }

    let z_size = queue.len()/2+1;
    let mut renames : HashMap<&str,String> = HashMap::new();
    for &name in queue.iter() {
        renames.insert(name, name.to_string());
    }

    while !queue.is_empty() {
        let next = queue.pop_front()?;
        match next.bytes().next()? {
            b'y' | b'z' => continue,
            _ => {},
        }
        // *(renamed.get_mut(next)?) = true;

        println!("Trying {next}");
        for g in gate_dependencies.get(next)?.iter() {
            let gate = &mut gates[*g];
            if renames.contains_key(gate.output) {
                continue;
            }

            println!(" - {:?}", gate);
            let logical = gate.logical_output(&renames);
            if logical.len() != 3 {
                println!(" - ERROR {logical}");
                continue;
            }
            // println!("Matching {} {:?} {} \t{:?}", renames[gate.input1], gate.operation, renames[gate.input2], gate);
            println!(" - {} => {logical}", gate.output);

            renames.insert(gate.output, logical);
            queue.push_back(gate.output);
            // println!("{} ({}) {:?} {} ({}) -> {} ({})", gate.input1, renames[gate.input1], gate.operation, gate.input2, renames[gate.input2], gate.output, renames[gate.output]);
        }

    }


    let mut i_set : HashSet<String> = HashSet::new();
    let mut c_set : HashSet<String> = HashSet::new();
    let mut m_set : HashSet<String> = HashSet::new();
    let mut n_set : HashSet<String> = HashSet::new();
    for i in 0..z_size {
        i_set.insert(format!("i{i:0>2}"));
        c_set.insert(format!("c{i:0>2}"));
        m_set.insert(format!("m{i:0>2}"));
        n_set.insert(format!("n{i:0>2}"));
    }
    for (&k, v) in renames.iter() {
        // println!("{k} -> {v}");
        i_set.remove(&v.to_string());
        c_set.remove(&v.to_string());
        m_set.remove(&v.to_string());
        n_set.remove(&v.to_string());
    }    

    println!("{}", renames.len());
    println!("Missing {:?}", i_set);
    println!("Missing {:?}", c_set);
    println!("Missing {:?}", m_set);
    println!("Missing {:?}", n_set);

    Some((i_set.len() + c_set.len() + m_set.len() + n_set.len()) as u64)
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
