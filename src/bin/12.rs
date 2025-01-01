use std::collections::{HashSet, VecDeque};


advent_of_code::solution!(12);

// 1449628 is too low
pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut v = Vec::new();
        for b in line.bytes() {
            v.push(b);
        }
        grid.push(v);
    }
    let grid_x = grid[0].len();
    let grid_y = grid.len();


    let mut price : u64 = 0;
    let mut search : [usize;2] = [0, 0];
    loop {
        let crop = grid[search[1]][search[0]];
        if crop > b'Z' {
            search[0] += 1;
            if search[0] == grid_x {
                search[0] = 0;
                search[1] += 1;
            }
            if search[1] == grid_y {
                break;
            }    
            continue;
        }

        // flood fill
        let mut perimeter : HashSet<[i32;2]> = HashSet::new();
        let mut area : HashSet<[i32;2]> = HashSet::new();
        let mut queue : VecDeque<[i32;2]> = VecDeque::from([ [search[0] as i32, search[1] as i32] ]);
        while !queue.is_empty() {
            let cur = queue.pop_front()?;
            if area.contains(&cur) {
                continue;
            }
            area.insert(cur);
            // println!("ahhhhh {:?}, {}", cur, grid[cur[1] as usize][cur[0] as usize]);
            grid[cur[1] as usize][cur[0] as usize] += 26;

            for delta in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
                let nb = [delta[0]+cur[0], delta[1]+cur[1]];                
                if nb[0] < 0 || nb[0] >= grid_x as i32 || nb[1] < 0 || nb[1] >= grid_y as i32 {
                    continue;
                }
                let nb_crop = grid[nb[1] as usize][nb[0] as usize];
                if nb_crop != crop {
                    continue;
                }
                
                queue.push_back(nb);
            }
            for delta in [[0, 0], [0, 1], [1, 0], [1, 1]] {
                let nb = [delta[0]+cur[0], delta[1]+cur[1]];                
                perimeter.insert(nb);
            }
        }
        let mut outer : usize = 0;
        for p in perimeter.iter() {
            let tl : bool = p[0] > 0 && p[1] > 0 && area.contains(&[p[0]-1, p[1]-1]);
            let tr : bool = p[0] < grid_x as i32 && p[1] > 0 && area.contains(&[p[0], p[1]-1]);
            let br : bool = p[0] < grid_x as i32 && p[1] < grid_y as i32 && area.contains(&[p[0], p[1]]);
            let bl : bool = p[0] > 0 && p[1] < grid_y as i32 && area.contains(&[p[0]-1, p[1]]);
            if tl && tr && br && bl {
                continue;
            }
            outer += 1;
        }
        price += (outer * area.len()) as u64;
        println!("{:?}, crop {}, area {}, perimeter {}", search, crop as char, area.len(), outer);
        
        // move search
        search[0] += 1;
        if search[0] == grid_x {
            search[0] = 0;
            search[1] += 1;
        }
        if search[1] == grid_y {
            break;
        }
    }

    Some(price)
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
