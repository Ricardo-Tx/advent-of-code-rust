use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(15);


pub fn part_one(input: &str) -> Option<u64> {
    #[derive(PartialEq)]
    enum Slot {
        Empty,
        Wall,
        Box,
        Unknown,
    }

    let mut grid : Vec<Vec<Slot>> = Vec::new();
    let mut pos : [i32;2] = [0, 0];
    let mut grid_x : usize = 0;
    let mut grid_y : usize = 0;

    let mut it = input.split('\n');
    loop {
        let line = it.next()?;
        if line.is_empty() {
            break;
        }

        let mut v = Vec::new();
        v.reserve(line.len());
        grid_x = 0;
        for b in line.bytes() {
            v.push(match b {
                b'.' | b'@' => Slot::Empty,
                b'#' => Slot::Wall,
                b'O' => Slot::Box,
                _ => Slot::Unknown,
            });
            if b == b'@' {
                pos = [grid_x as i32, grid_y as i32];
            }
            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
    }

    loop {
        let line = match it.next() {
            Some(x) => {
                if x.is_empty() {
                    continue;
                }
                x
            },
            None => {
                break;
            }
        };

        for b in line.bytes() {
            let delta : [i32;2] = match b {
                b'>' => [1, 0],
                b'^' => [0, -1],
                b'<' => [-1, 0],
                b'v' => [0, 1],
                _ => [0, 0],
            };

            let target = [ pos[0] + delta[0], pos[1] + delta[1] ];

            match grid[target[1] as usize][target[0] as usize] {
                Slot::Empty => {
                    pos = target;
                },
                Slot::Box => {
                    let mut end_target = target;
                    let end_slot : &Slot = loop {
                        end_target = [end_target[0] + delta[0], end_target[1] + delta[1]];
                        let end_slot = &grid[end_target[1] as usize][end_target[0] as usize];
                        match end_slot {
                            Slot::Empty | Slot::Wall => break end_slot,
                            _ => continue,
                        }
                    };

                    if *end_slot == Slot::Empty {
                        grid[end_target[1] as usize][end_target[0] as usize] = Slot::Box;
                        grid[target[1] as usize][target[0] as usize] = Slot::Empty;
                        pos = target;
                    }
                },
                _ => {},
            }
        }
    }


    let mut total : u64 = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == Slot::Box {
                total += (x + 100*y) as u64;
            }
        }
    } 

    Some(total)   
}

// 1192705 is too low
pub fn part_two(input: &str) -> Option<u64> {
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    enum Slot {
        Empty,
        Wall,
        BoxLeft,
        BoxRight,
    }

    let mut grid : Vec<Vec<Slot>> = Vec::new();
    let mut pos : [i32;2] = [0, 0];
    let mut grid_x : usize = 0;
    let mut grid_y : usize = 0;

    let mut it = input.split('\n');
    loop {
        let line = it.next()?;
        if line.is_empty() {
            break;
        }

        let mut v = Vec::new();
        v.reserve(line.len());
        grid_x = 0;
        for b in line.bytes() {
            match b {
                b'.' | b'@' => {
                    v.push(Slot::Empty);
                    v.push(Slot::Empty);
                },
                b'#' => {
                    v.push(Slot::Wall);
                    v.push(Slot::Wall);
                },
                b'O' => {
                    v.push(Slot::BoxLeft);
                    v.push(Slot::BoxRight);
                },
                _ => {}
            };
            if b == b'@' {
                pos = [grid_x as i32 * 2, grid_y as i32];
            }
            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
    }

    loop {
        let line = match it.next() {
            Some(x) => {
                if x.is_empty() {
                    continue;
                }
                x
            },
            None => {
                break;
            }
        };

        for b in line.bytes() {
            let delta : [i32;2] = match b {
                b'>' => [1, 0],
                b'^' => [0, -1],
                b'<' => [-1, 0],
                b'v' => [0, 1],
                _ => [0, 0],
            };

            let target = [ pos[0] + delta[0], pos[1] + delta[1] ];
            let target_slot = &grid[target[1] as usize][target[0] as usize];
            match target_slot {
                Slot::Empty => {
                    pos = target;
                },
                Slot::BoxLeft | Slot::BoxRight => {
                    match b {
                        b'>' | b'<' => {
                            let mut end_target = target;
                            let end_slot : &Slot = loop {
                                end_target = [end_target[0] + delta[0], end_target[1] + delta[1]];
                                let end_slot = &grid[end_target[1] as usize][end_target[0] as usize];
                                match end_slot {
                                    Slot::Empty | Slot::Wall => break end_slot,
                                    _ => continue,
                                }
                            };
        
                            if *end_slot == Slot::Empty {
                                let mut cur_slot = if b == b'>' { Slot::BoxRight } else { Slot::BoxLeft };
                                while end_target[0] != target[0] {
                                    grid[end_target[1] as usize][end_target[0] as usize] = cur_slot;
                                    cur_slot = if cur_slot == Slot::BoxLeft { Slot::BoxRight } else { Slot::BoxLeft };
                                    end_target[0] -= delta[0];
                                }
                                grid[target[1] as usize][target[0] as usize] = Slot::Empty;
                                pos = target;
                            }
                        },
                        b'^' | b'v' => {
                            let mut boxes : Vec<[i32;2]> = vec![
                                if *target_slot == Slot::BoxLeft { target } else { [target[0]-1, target[1]] }
                            ];
                            let mut l = 0;
                            let mut movable : bool = true;
                            while l < boxes.len() {
                                let b = boxes[l];

                                let prop_left = &grid[(b[1] + delta[1]) as usize][b[0] as usize];
                                match prop_left {
                                    Slot::BoxLeft => {
                                        boxes.push([b[0], b[1]+delta[1]]);
                                        l += 1;
                                        continue;
                                    },
                                    Slot::BoxRight => {
                                        boxes.push([b[0]-1, b[1]+delta[1]]);
                                    },
                                    Slot::Wall => {
                                        movable = false;
                                        break;
                                    },
                                    _ => {},
                                }
                                
                                let prop_right = &grid[(b[1] + delta[1]) as usize][(b[0]+1) as usize];
                                match prop_right {
                                    Slot::BoxLeft => {
                                        boxes.push([b[0]+1, b[1]+delta[1]]);
                                    },
                                    Slot::Wall => {
                                        movable = false;
                                        break;
                                    },
                                    _ => {},
                                }

                                l += 1;
                            }

                            if movable {
                                for p in boxes.iter().rev() {                                    
                                    grid[(p[1]+delta[1]) as usize][p[0] as usize] = Slot::BoxLeft;
                                    grid[(p[1]+delta[1]) as usize][(p[0]+1) as usize] = Slot::BoxRight;
                                    grid[p[1] as usize][p[0] as usize] = Slot::Empty;
                                    grid[p[1] as usize][(p[0]+1) as usize] = Slot::Empty;
                                }
                                pos = target;
                            }
                        }
                        _ => {}
                    }
                },
                _ => {},
            }



            // // print
            // for (y, line) in grid.iter().enumerate() {
            //     for (x, item) in line.iter().enumerate() {
            //         if pos[0] as usize == x && pos[1] as usize == y {
            //             print!("@");
            //         } else {
            //             let ch = match item {
            //                 Slot::BoxLeft => '[',
            //                 Slot::BoxRight => ']',
            //                 Slot::Empty => '.',
            //                 Slot::Wall => '#',
            //             };
            //             print!("{ch}");
            //         }
            //     }
            //     println!();
            // }
            // println!();
        }
    }


    let mut total : u64 = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == Slot::BoxLeft {
                total += (x + 100*y) as u64;
            }
        }
    } 
    
    Some(total)
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
