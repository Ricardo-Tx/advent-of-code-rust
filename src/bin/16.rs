use std::collections::VecDeque;
use advent_of_code::point_to_str;


advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u64>> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut v = Vec::new();
        for b in line.bytes() {
            v.push(match b {
                b'#' => u64::MAX,
                b'.' | b'S' | b'E' => 0,
                _ => 0,
            });
        }
        grid.push(v);
    }
    
    let grid_x = grid[0].len();
    let grid_y = grid.len();

    let mut queue : VecDeque<([i32;2], [i32;2])> = VecDeque::from([ ([1 as i32, (grid_y-2) as i32], [1, 0]) ]);
    while !queue.is_empty() {
        let pop = queue.pop_front()?;
        let cur = pop.0;
        let dir = pop.1;
        let score = grid[cur[1] as usize][cur[0] as usize];

        for delta in [ [1, 0], [-1, 0], [0, 1], [0, -1] ] {
            if delta[0] == -dir[0] && delta[1] == -dir[1] {
                continue;
            } 
            let target = [cur[0]+delta[0], cur[1]+delta[1]];
            let target_score = &mut grid[target[1] as usize][target[0] as usize];            
            if *target_score < u64::MAX {
                let turn = dir[0]*delta[0] == 0 && dir[1]*delta[1] == 0;
                let new_score = score + 1 + (turn as u64) * 1000;
                if *target_score == 0 || new_score < *target_score {
                    *target_score = new_score;
                    queue.push_back( (target, delta) );
                }
            }
        }
    }

    // println!("{}", grid[15][1]);
    // println!("{}", grid[14][1]);
    // println!("{}", grid[13][1]);
    // println!("{}", grid[12][1]);
    // println!("{}", grid[11][1]);
    // println!("{}", grid[10][1]);
    // println!("{}...\n", grid[9][1]);
    // println!("{} {}", grid[5][1], point_to_str(&[1,5]));
    // println!("{} {}", grid[5][3], point_to_str(&[3,5]));
    // println!("{} {}", grid[15][3], point_to_str(&[3,15]));
    // println!("{} {}", grid[15][5], point_to_str(&[5,15]));
    // println!("{} {}", grid[11][5], point_to_str(&[5,11]));
    // println!("{} {}", grid[11][7], point_to_str(&[7,11]));
    // println!("{} {}", grid[9][7], point_to_str(&[7,9]));
    // println!("{} {}", grid[9][11], point_to_str(&[11,9]));
    // println!("{} {}", grid[7][11], point_to_str(&[11,7]));
    // println!("{} {}...", grid[7][14], point_to_str(&[14,7]));
    // println!("{} {}", grid[7][15], point_to_str(&[15,7]));
    // println!("{} {}...", grid[6][15], point_to_str(&[15,6]));
    // println!("{} {}", grid[1][15], point_to_str(&[15,1]));

    for y in 0..grid_y {
        for x in 0..grid_x {
            if grid[y][x] == u64::MAX {
                print!("##### ");
            } else {
                print!("{:<6}", grid[y][x]);
            }
        }
        println!();
    }

    Some(grid[1][grid_x-2])
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
