use std::collections::HashMap;

advent_of_code::solution!(20);

const SAVE : usize = 100;

// 1050656 is too high
// example with SAVE=50 should be 285
pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u64>> = Vec::new();
    let mut init_pos : [usize;2] = [0, 0];
    let mut grid_x = 0;
    let mut grid_y = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        
        grid_x = 0;
        let mut v = Vec::new();
        for b in line.bytes() {
            v.push(match b {
                b'#' => u64::MAX,
                b'.' | b'S' | b'E' => 0,
                _ => 0,
            });
            if b == b'S' {
                init_pos = [grid_x, grid_y];
            }
            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
        grid_x = line.len();
    }
    
    // distance filling
    let mut pos = init_pos;
    let mut progress : u64 = 0;
    loop {
        progress += 1;
        grid[pos[1]][pos[0]] = progress;
        let search : bool = 'outer: {
            for delta in [ [0, 1], [0, -1], [1, 0], [-1, 0] ] {
                let next = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
                let next = [next[0] as usize, next[1] as usize];
                
                if grid[next[1]][next[0]] > 0 {
                    continue;
                }
                pos = next;
                break 'outer true;
            }
            false
        };
        if !search {
            break;
        }
    }

    // cheat finding
    pos = init_pos;
    let mut best : u64 = 0;
    loop {
        let mut pos_next : Option<[usize;2]> = None;
        let dist = grid[pos[1]][pos[0]];
        for delta in [ [0, 1], [0, -1], [1, 0], [-1, 0] ] {
            let nb = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
            let nb = [nb[0] as usize, nb[1] as usize];
            
            if pos_next == None && grid[nb[1]][nb[0]] == dist+1 {
                pos_next = Some(nb);
            }

            if nb[0] == 0 || nb[0] == grid_x-1 || nb[1] == 0 || nb[1] == grid_y-1 {
                continue;
            }

            let warp = [pos[0] as i32 + delta[0]*2, pos[1] as i32 + delta[1]*2];
            let warp = [warp[0] as usize, warp[1] as usize];
            let warp_dist = grid[warp[1]][warp[0]];
            if warp_dist < u64::MAX && warp_dist > dist+2 {
                let save = warp_dist-dist-2;
                if save >= SAVE as u64 {
                    best += 1;
                }
            }
        }

        if let Some(x) = pos_next {
            pos = x;
        } else {
            break;
        }
    }

    Some(best)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u64>> = Vec::new();
    let mut init_pos : [usize;2] = [0, 0];
    let mut grid_x = 0;
    let mut grid_y = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        
        grid_x = 0;
        let mut v = Vec::new();
        for b in line.bytes() {
            v.push(match b {
                b'#' => u64::MAX,
                b'.' | b'S' | b'E' => 0,
                _ => 0,
            });
            if b == b'S' {
                init_pos = [grid_x, grid_y];
            }
            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
        grid_x = line.len();
    }
    
    // distance filling
    let mut pos = init_pos;
    let mut path : Vec<[usize;2]> = Vec::new();
    loop {
        path.push(pos);
        let search : bool = 'outer: {
            for delta in [ [0, 1], [0, -1], [1, 0], [-1, 0] ] {
                let next = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
                let next = [next[0] as usize, next[1] as usize];
                
                if grid[next[1]][next[0]] == u64::MAX || (path.len() > 1 && path[path.len()-2] == next) {
                    continue;
                }
                pos = next;
                break 'outer true;
            }
            false
        };
        if !search {
            break;
        }
    }
    
    // let mut cheats : HashMap<usize,u64> = HashMap::new();
    let mut best : u64 = 0;
    for save in (SAVE..path.len()).rev() {
        for i in 0..(path.len()-save) {
            let dist = path[i][0].abs_diff(path[i+save][0]) + path[i][1].abs_diff(path[i+save][1]);
            let real_save = save - dist;
            // println!("Cheat between {}th and {}th has positions {} units apart. Saves {} ps", i, i+save, dist, real_save);

            if dist > 20 {
                continue;
            }

            // if let Some(opt) = cheats.get_mut(&real_save) {
            //     *opt += 1;
            // } else {
            //     cheats.insert(real_save, 1);
            // }
            if real_save >= SAVE {
                best += 1;
            }
        }
    }
    // println!("{:?}", cheats);
    // let mut cheat_sort : Vec<(&usize, &u64)> = cheats.iter().collect();
    // cheat_sort.sort();
    // for (k, v) in cheat_sort {
    //     println!("- There are {v} cheats\tthat save {k} picoseconds.");
    // }

    Some(best)
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
