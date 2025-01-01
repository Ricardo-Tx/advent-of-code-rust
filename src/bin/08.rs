use std::collections::{HashMap, HashSet};
use advent_of_code::point_to_str;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut antinodes : HashSet<[i32;2]> = HashSet::new();
    let mut antennas : HashMap<u8,Vec<[i32;2]>> = HashMap::new();
    let mut grid_x : i32 = 0;
    let mut grid_y : i32 = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        
        grid_x = 0;
        for b in line.bytes() {
            grid_x += 1;
            if b == b'.' {
                continue;
            }
            
            if antennas.contains_key(&b) {
                antennas.get_mut(&b)?.push([grid_x-1, grid_y]);
            } else {
                antennas.insert(b, vec![[grid_x-1, grid_y]]);
            }
        }

        grid_y += 1;
        grid_x = line.len() as i32;
    }

    for a in antennas.values() {
        // println!("{a:?}");
        for i in 0..a.len()-1 {
            for j in i+1..a.len() {
                let a1 = a[i];
                let a2 = a[j];

                let an1 = [2*a1[0]-a2[0], 2*a1[1]-a2[1]];
                if an1[0] >= 0 && an1[0] < grid_x && an1[1] >= 0 && an1[1] < grid_y {
                    if antinodes.insert(an1) {
                        // println!("{}", point_to_str(&an1))
                    }
                }
                
                let an2 = [2*a2[0]-a1[0], 2*a2[1]-a1[1]];
                if an2[0] >= 0 && an2[0] < grid_x && an2[1] >= 0 && an2[1] < grid_y {
                    if antinodes.insert(an2) {
                        // println!("{}", point_to_str(&an2));
                    }
                }
            }
        }
    }

    // for a in &antinodes {
    //     println!("{}", point_to_str(&a));
    // }

    Some(antinodes.len() as u64)
}

// 1136 is too low
pub fn part_two(input: &str) -> Option<u64> {
    let mut antinodes : HashSet<[i32;2]> = HashSet::new();
    let mut antennas : HashMap<u8,Vec<[i32;2]>> = HashMap::new();
    let mut grid_x : i32 = 0;
    let mut grid_y : i32 = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        
        grid_x = 0;
        for b in line.bytes() {
            grid_x += 1;
            if b == b'.' {
                continue;
            }
            
            if antennas.contains_key(&b) {
                antennas.get_mut(&b)?.push([grid_x-1, grid_y]);
            } else {
                antennas.insert(b, vec![[grid_x-1, grid_y]]);
            }
        }
    
        grid_y += 1;
        grid_x = line.len() as i32;
    }
    
    for a in antennas.values() {
        // println!("{a:?}");
        for i in 0..a.len()-1 {
            for j in i+1..a.len() {
                let a1 = a[i];
                let a2 = a[j];
    
                // let mut an1 = [2*a1[0]-a2[0], 2*a1[1]-a2[1]];
                let mut an1 = [a1[0], a1[1]];
                while an1[0] >= 0 && an1[0] < grid_x && an1[1] >= 0 && an1[1] < grid_y {
                    antinodes.insert(an1);
                    an1[0] += a1[0] - a2[0];
                    an1[1] += a1[1] - a2[1];
                }
                
                let mut an2 = [a2[0], a2[1]];
                while an2[0] >= 0 && an2[0] < grid_x && an2[1] >= 0 && an2[1] < grid_y {
                    antinodes.insert(an2);
                    an2[0] += a2[0] - a1[0];
                    an2[1] += a2[1] - a1[1];
                }                
            }
        }
    }
    
    // for a in &antinodes {
    //     println!("{}", point_to_str(&a));
    // }
    
    Some(antinodes.len() as u64)
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
