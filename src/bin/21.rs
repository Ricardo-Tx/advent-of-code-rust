use std::{collections::{HashMap, VecDeque}, mem::swap};

advent_of_code::solution!(21);

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+


// v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

// const NUMERIC_KEYPAD : [[u8;3];4] = [
//     [b'7', b'8', b'9'],
//     [b'4', b'5', b'6'],
//     [b'1', b'2', b'3'],
//     [b'X', b'0', b'A'],
// ];

// const DIRECT_KEYPAD : [[u8;3];2] = [
//     [b'^', b'A', b'X'],
//     [b'<', b'v', b'>'],
// ];
#[derive(Debug)]
struct Task {
    category: u8,
    times: u64,
}

// 164230 is too high
pub fn part_one(input: &str) -> Option<u64> {
    let numeric_keypad : HashMap<u8,[usize;2]> = HashMap::from([
        (b'A', [2, 3]),
        (b'0', [1, 3]),
        (b'1', [0, 2]),
        (b'2', [1, 2]),
        (b'3', [2, 2]),
        (b'4', [0, 1]),
        (b'5', [1, 1]),
        (b'6', [2, 1]),
        (b'7', [0, 0]),
        (b'8', [1, 0]),
        (b'9', [2, 0]),
    ]);

    let direct_keypad : HashMap<u8,[usize;2]> = HashMap::from([
        (b'A', [2, 0]),
        (b'^', [1, 0]),
        (b'v', [1, 1]),
        (b'>', [2, 1]),
        (b'<', [0, 1]),
    ]);

    let numeric_tasks = |tasks: &mut VecDeque<Task>, robot: &mut [usize;2]| {
        for _ in 0..tasks.len() {
            let val = match tasks.pop_front() {
                Some(x) => x,
                None => break,
            };
            let target = numeric_keypad[&val.category];
            // println!("From {:?} to {:?}", door_robot, target);
            let delta = [target[0] as i32 - robot[0] as i32, target[1] as i32 - robot[1] as i32];
            
            if delta[0] != 0 {
                tasks.push_back(Task {
                    category: if delta[0] > 0 { b'>' } else { b'<' },
                    times: delta[0].abs() as u64,
                });
            }
            if delta[1] != 0 {
                tasks.push_back(Task {
                    category: if delta[1] > 0 { b'v' } else { b'^' },
                    times: delta[1].abs() as u64,
                });
            }

            // if going through the deadzone
            let i_len = tasks.len();
            if delta[0] < 0 && robot[1] == 3 && delta[1] < 0 && target[0] == 0 {
                tasks.swap(i_len-2, i_len-1);
            }
            
            if tasks[i_len-1].category == b'A' {
                tasks[i_len-1].times += 1;
            } else {
                tasks.push_back(Task {
                    category: b'A',
                    times: 1,
                });
            }
            *robot = target;
        }
    };

    let direct_tasks = |tasks: &mut VecDeque<Task>, robot: &mut [usize;2]| {
        for _ in 0..tasks.len() {
            let val = match tasks.pop_front() {
                Some(x) => x,
                None => break,
            };
            let target = direct_keypad[&val.category];
            // println!("From {:?} to {:?}", robot, target);
            let delta = [target[0] as i32 - robot[0] as i32, target[1] as i32 - robot[1] as i32];
            
            if delta[0] != 0 {
                tasks.push_back(Task {
                    category: if delta[0] > 0 { b'>' } else { b'<' },
                    times: delta[0].abs() as u64,
                });
            }
            if delta[1] != 0 {
                tasks.push_back(Task {
                    category: if delta[1] > 0 { b'v' } else { b'^' },
                    times: delta[1].abs() as u64,
                });
            }
            
            // if going through the deadzone
            let i_len = tasks.len();
            if delta[0] < 0 && robot[1] == 0 && delta[1] > 0 && target[0] == 0 {
                tasks.swap(i_len-2, i_len-1);
            }
            
            tasks.push_back(Task {
                category: b'A',
                ..val
            });
            *robot = target;
        }
    };

    let print_tasks = |tasks: &VecDeque<Task>| {
        for e in tasks.iter() {
            for _ in 0..e.times {
                print!("{}", e.category as char);
            }
        }
        println!();
    };

    let mut complexities : u64 = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut instr : VecDeque<Task> = VecDeque::new();
        for b in line.bytes() {
            instr.push_back(Task {
                category: b,
                times: 1,
            });
        }
        // arms of the robots
        let mut door_robot : [usize;2] = [2, 3];
        let mut press_robot : [usize;2] = [2, 0];
        let mut rad_robot : [usize;2] = [2, 0];
        let mut cold_robot : [usize;2] = [2, 0];
    
        print!("{line}: ");
        numeric_tasks(&mut instr, &mut door_robot);
        print_tasks(&instr);
                
        direct_tasks(&mut instr, &mut press_robot);
        print_tasks(&instr);
        
        direct_tasks(&mut instr, &mut rad_robot);
        print_tasks(&instr);
        
        let mut presses : u64 = 0;
        for t in &instr {
            presses += t.times;
        }
        complexities += presses * line[..line.len()-1].parse::<u64>().expect("duh");
        println!(" - moves: {presses}");
        println!();
    }
    
    Some(complexities)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
