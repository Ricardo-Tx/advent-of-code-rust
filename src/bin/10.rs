use std::collections::{HashSet, VecDeque};
use advent_of_code::point_to_str;

advent_of_code::solution!(10);

struct Node {
    value: u8,
    origins: HashSet<[usize;2]>,
}


// https://stackoverflow.com/questions/74873575/how-to-sort-or-reverse-vecdeque-without-make-contiguous
fn sort_vecdeque<T: Ord>(x: &mut VecDeque<T>) {
    x.rotate_right(x.as_slices().1.len());
    assert!(x.as_slices().1.is_empty());
    x.as_mut_slices().0.sort();
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<Node>> = Vec::new();
    let mut grid_x : usize = 0;
    let mut grid_y : usize = 0;
    let mut init_wavefront : HashSet<[usize;2]> = HashSet::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut v = Vec::new();
        v.reserve(line.len());
        grid_x = 0;
        for b in line.bytes() {
            let value = b - 48;
            let pos = [grid_x, grid_y];
            let mut node = Node {
                value,
                origins: HashSet::new(),
            };
            
            if value == 0 {
                init_wavefront.insert(pos);
                node.origins.insert(pos);
            }
            v.push(node);
            
            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
        grid_x = line.len();
    }


    let mut val = 0;
    let mut wavefront : HashSet<[usize;2]> = init_wavefront.clone();
    while val < 9 {
        // println!("Value {val}");
        
        let mut new_wavefront : HashSet<[usize;2]> = HashSet::new();
        for pos in wavefront.iter() {
            let (pos_value, pos_origins) = {
                let node = &grid[pos[1]][pos[0]];
                (node.value, node.origins.clone())
            };
            // println!("Doing {}\tval {} {:?}", point_to_str(&pos), pos_value, pos_origins);

            // iterate through neighbors
            for delta in [[-1,0], [0,1], [1,0], [0,-1]] {
                let nb = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
                if nb[0] < 0 || nb[0] >= grid_x as i32 || nb[1] < 0 || nb[1] >= grid_y as i32 {
                    continue;   // outside of grid
                } 
    
                let nb = [nb[0] as usize, nb[1] as usize];
                let nb_node = &mut grid[nb[1]][nb[0]];
                if nb_node.value == val+1 {
                    new_wavefront.insert(nb);
    
                    // assign child
                    for o in pos_origins.iter() {
                        nb_node.origins.insert(*o);
                    }
                }
            }
        }
        val += 1;
        wavefront = new_wavefront;
        // println!();
    }

    let mut scores = 0u64;
    for pos in wavefront.iter() {
        scores += grid[pos[1]][pos[0]].origins.len() as u64;
    }
    Some(scores)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<Node>> = Vec::new();
    let mut grid_x : usize = 0;
    let mut grid_y : usize = 0;
    let mut init_wavefront : VecDeque<[usize;2]> = VecDeque::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut v = Vec::new();
        v.reserve(line.len());
        grid_x = 0;
        for b in line.bytes() {
            let value = b - 48;
            v.push(Node {
                value,
                origins: HashSet::new(),
            });

            if value == 0 {
                init_wavefront.push_back([grid_x, grid_y]);
            }

            grid_x += 1;
        }
        grid.push(v);
        grid_y += 1;
        grid_x = line.len();
    }


    let mut val = 0;
    let mut wavefront : VecDeque<[usize;2]> = init_wavefront.clone();
    while val < 9 {
        // println!("Value {val}");
        sort_vecdeque(&mut wavefront);
        let mut cur_len = wavefront.len();
        
        while cur_len > 0 {
            let pos = wavefront.pop_front()?;
            cur_len -= 1;
            let mut repeats = 1;
            while !wavefront.is_empty() && wavefront[0] == pos {
                cur_len -= 1;
                repeats += 1;
                wavefront.pop_front();
            }
            // println!("Doing {} x{}\tval {}", point_to_str(&pos), repeats, grid[pos[1]][pos[0]].value);

            // iterate through neighbors
            for delta in [[-1,0], [0,1], [1,0], [0,-1]] {
                let nb = [pos[0] as i32 + delta[0], pos[1] as i32 + delta[1]];
                if nb[0] < 0 || nb[0] >= grid_x as i32 || nb[1] < 0 || nb[1] >= grid_y as i32 {
                    continue;   // outside of grid
                } 
    
                let nb = [nb[0] as usize, nb[1] as usize];
                if grid[nb[1]][nb[0]].value == val+1 {
                    // println!("{}", grid[nb[1]][nb[0]].value);
                    for _ in 0..repeats {
                        wavefront.push_back(nb);
                    }    
                }
            }
        }
        val += 1;
        // println!();
    }
    Some(wavefront.len() as u64)
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
