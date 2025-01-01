use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(18);

const GRID_X : usize = 71;
const GRID_Y : usize = 71;
// const GRID_X : usize = 7;
// const GRID_Y : usize = 7;

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : [[u64;GRID_X];GRID_Y] = [[0;GRID_X];GRID_Y];
    for (i, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 1024 {
            break;
        }
        let mut it = line.split(',');
        let x : usize = it.next()?.parse().expect("bruh");
        let y : usize = it.next()?.parse().expect("bruh");
        grid[y][x] = u64::MAX;
    }

    let mut queue : VecDeque<[usize;2]> = VecDeque::from([[0, 0]]);
    'outer: while !queue.is_empty() {
        // println!("{:?}", queue);
        let pos = queue.pop_front()?;
        let score = grid[pos[1]][pos[0]];
        for delta in [ [1, 0], [-1, 0], [0, 1], [0, -1] ] {
            let next_pos = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
            if next_pos[0] < 0 || next_pos[0] >= GRID_X as i32 || next_pos[1] < 0 || next_pos[1] >= GRID_Y as i32 {
                continue;
            }
            let next_pos = [next_pos[0] as usize, next_pos[1] as usize];
            let next_score = grid[next_pos[1]][next_pos[0]];
            if next_score > 0 {
                continue;
            }

            grid[next_pos[1]][next_pos[0]] = score + 1;
            if next_pos == [GRID_X-1, GRID_Y-1] {
                break 'outer;
            } else {
                queue.push_back(next_pos);
            }
        }
    }

    let end = grid[GRID_Y-1][GRID_X-1];
    // for y in 0..GRID_Y {
    //     for x in 0..GRID_X {
    //         if y == 0 && x == 0 {
    //             print!("0   ");
    //         } else if grid[y][x].0 > end {
    //             print!("{:<4}", grid[y][x].1);
    //         } else {
    //             print!("#   ");
    //         }
    //     }
    //     println!();
    // }

    Some(end)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points : HashMap<[i32;2],u8> = HashMap::new();
    for (k, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut it = line.split(',');
        let x : i32 = it.next()?.parse().expect("bruh");
        let y : i32 = it.next()?.parse().expect("bruh");
        let cur = [x, y];

        // touching edges
        let mut v : u8 = 0;
        if x == 0 {
            v |= 4;
        } else if x as usize == GRID_X-1 {
            v |= 1;
        }
        if y == 0 {
            v |= 2;
        } else if y as usize == GRID_Y-1 {
            v |= 8;
        }
        points.insert(cur, v);


        let mut vct : Vec<[i32;2]> = Vec::from([cur]);
        let mut i : usize = 0;
        let mut com : u8 = v;
        while i < vct.len() {
            let pt = vct[i];
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let next = [pt[0]+x, pt[1]+y];
                    if points.contains_key(&next) && !vct.contains(&next) {
                        vct.push(next);
                        com |= points.get(&next)?;
                    }
                }
            }
            i += 1;
        }

        if (com >> 2) > 0 && (com & 3) > 0 {
            println!("ANSWER: {:?}", cur);
            break;
        }

        for p in vct {
            *points.get_mut(&p)? |= com;
        }


        // println!("TEST {}: {:?}", k, cur);
        // for y in 0..GRID_Y {
        //     for x in 0..GRID_X {
        //         if points.contains_key(&[x as i32, y as i32]) {
        //             print!("{:<3}", points.get(&[x as i32, y as i32])?);
        //         } else {
        //             print!(".  ");
        //         }
        //     }
        //     println!();
        // }
        // println!();

        // if (com >> 2) > 0 && (com & 3) > 0 {
        //     println!("{} {:?}", k, cur);
        //     break;
        // }
    }
        
    Some(42)
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
