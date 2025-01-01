use std::collections::{HashMap, HashSet};
use advent_of_code::point_to_str;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u8>> = Vec::new();
    let mut pos : [usize;2] = [0, 0];
    let mut rot : u8 = 1;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        grid.push(Vec::new());
        let size = grid.len();
        for (i, b) in line.bytes().enumerate() {
            let num = match b {
                b'.' => 0,
                b'#' => 1,
                b'^' => {
                    pos = [i, size-1];
                    2
                },
                _ => 42,
            };
            grid.get_mut(size-1)?.push(num);
        }
    }


    let mut walk : u64 = 1;
    loop {
        let movement = |p: &mut [usize;2], dir: u8| {
            match dir {
                0 => {
                    if p[0] == grid[0].len()-1 {
                        return false;
                    }
                    p[0] += 1
                },
                1 => {
                    if p[1] == 0 {
                        return false;
                    }
                    p[1] -= 1
                },
                2 => {
                    if p[0] == 0 {
                        return false;
                    }
                    p[0] -= 1
                },
                3 => {
                    if p[1] == grid.len()-1 {
                        return false;
                    }
                    p[1] += 1
                },
                _ => {}
            }
            true
        };

        let res = movement(&mut pos, rot);
        if !res {
            break;
        }
        if grid[pos[1]][pos[0]] == 1 {
            // blocked. go back and rotate
            movement(&mut pos,(rot+2)%4);
            rot = (rot+3)%4;
        } else if grid[pos[1]][pos[0]] == 0 {
            // walk through new spot. add to walk and mark as walked
            grid[pos[1]][pos[0]] = 2;
            walk += 1;
        }
        // walk normally
    }

    Some(walk)
}


// 420 is too low
pub fn part_two(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u8>> = Vec::new();
    let mut pos : [usize;2] = [0, 0];
    let mut rot : u8 = 1;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        grid.push(Vec::new());
        let size = grid.len();
        for (i, b) in line.bytes().enumerate() {
            let num = match b {
                b'.' => 0,
                b'#' => 1,
                b'^' => {
                    pos = [i, size-1];
                    0
                },
                _ => 42,
            };
            grid.get_mut(size-1)?.push(num);
        }
    }


    let mut loops : u64 = 0;
    let mut targets : HashMap<u8, HashSet<[usize;2]>> = HashMap::from([
        (0, HashSet::new()),
        (1, HashSet::new()),
        (2, HashSet::new()),
        (3, HashSet::new()),
    ]);

    loop {
        let movement = |p: &mut [usize;2], dir: u8| {
            match dir {
                0 => {
                    if p[0] == grid[0].len()-1 {
                        return false;
                    }
                    p[0] += 1
                },
                1 => {
                    if p[1] == 0 {
                        return false;
                    }
                    p[1] -= 1
                },
                2 => {
                    if p[0] == 0 {
                        return false;
                    }
                    p[0] -= 1
                },
                3 => {
                    if p[1] == grid.len()-1 {
                        return false;
                    }
                    p[1] += 1
                },
                _ => {}
            }
            true
        };

        let res = movement(&mut pos, rot);
        if !res {
            break;
        }
        if grid[pos[1]][pos[0]] == 1 {
            // blocked. go back and rotate
            movement(&mut pos,(rot+2)%4);
            rot = (rot+3)%4;

            let back = (rot+3)%4;
            let catch = (back+3)%4;
            let mut target = pos;
            while movement(&mut target, back) && grid[target[1]][target[0]] == 0 {
                targets.get_mut(&catch)?.insert(target);
            }
        } else if grid[pos[1]][pos[0]] == 0 {
            // walk. check if is going through target
            if targets.get(&rot)?.contains(&pos) {
                loops += 1;
                // println!("hit a target at {}", point_to_str(&pos));
            }
        }
    }

    Some(loops)
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
