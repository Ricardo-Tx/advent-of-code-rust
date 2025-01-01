advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    position : [i64;2],
    velocity : [i64;2],
}

const GRID_X : i64 = 101;
const GRID_Y : i64 = 103;

impl Robot {
    fn normalize(&mut self) {
        // 101x103
        while self.velocity[0] < 0 {
            self.velocity[0] += GRID_X;
        }
        self.velocity[0] %= GRID_X;
        
        while self.velocity[1] < 0 {
            self.velocity[1] += GRID_Y;
        }
        self.velocity[1] %= GRID_Y;
    }

    fn evolve(&mut self, seconds : i64) {
        self.position[0] += self.velocity[0] * seconds;
        self.position[0] %= GRID_X;
        
        self.position[1] += self.velocity[1] * seconds;
        self.position[1] %= GRID_Y;
    }

    fn quadrant(&self) -> Option<usize> {
        if self.position[0]*2+1 == GRID_X || self.position[1]*2+1 == GRID_Y {
            return None;
        }
        let mut u = 2*((self.position[1]*2 / GRID_Y > 0) as usize) + (self.position[0]*2 / GRID_X > 0) as usize;
        if u < 2 {
            u = 1-u;
        }
        Some(u)
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut quadrants : [u64;4] = [0, 0, 0, 0]; 
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut nums : [i64;4] = [0, 0, 0, 0];
        let mut i : usize = 3;
        let mut neg : bool = false;

        for b in line.bytes() {
            let back = &mut nums[i];

            if b == b',' || b == b'=' {
                if neg {
                    *back *= -1;
                }
                neg = false;
                i = (i+1)%4;
            } else if b >= b'0' && b <= b'9' {
                *back *= 10;
                *back += (b-48) as i64;
            } else if *back == 0 && b == b'-' {
                neg = true;
            }
        }
        if neg {
            nums[3] *= -1;
        }


        let mut robot = Robot {
            position: [nums[0], nums[1]],
            velocity: [nums[2], nums[3]],
        };   

        // println!("{:?}", robot);
        robot.normalize();
        robot.evolve(100);
        // println!("{:?} -> {:?}", robot, robot.quadrant());
        // println!();

        if let Some(q) = robot.quadrant() {
            quadrants[q] += 1;
        }
    }
    Some(quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3])
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut robots : Vec<Robot> = Vec::new();
    let mut grid : [[u8;GRID_X as usize];GRID_Y as usize] = [[0;GRID_X as usize];GRID_Y as usize];
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut nums : [i64;4] = [0, 0, 0, 0];
        let mut i : usize = 3;
        let mut neg : bool = false;

        for b in line.bytes() {
            let back = &mut nums[i];

            if b == b',' || b == b'=' {
                if neg {
                    *back *= -1;
                }
                neg = false;
                i = (i+1)%4;
            } else if b >= b'0' && b <= b'9' {
                *back *= 10;
                *back += (b-48) as i64;
            } else if *back == 0 && b == b'-' {
                neg = true;
            }
        }
        if neg {
            nums[3] *= -1;
        }


        let mut robot = Robot {
            position: [nums[0], nums[1]],
            velocity: [nums[2], nums[3]],
        };   
        grid[robot.position[1] as usize][robot.position[0] as usize] += 1;
        
        // println!("{:?}", robot);
        robot.normalize();
        robots.push(robot);
    }
    
    let mut it = 0;
    let mut secs : u64 = 0;
    for _ in 0..10000000 {
        for r in robots.iter_mut() {
            grid[r.position[1] as usize][r.position[0] as usize] -= 1;
            r.evolve(1);
            grid[r.position[1] as usize][r.position[0] as usize] += 1;
        }
        let mut occupied : u64 = 0;
        for line in grid {
            for val in line {
                // let ch : char = 'outer: {
                //     if val == 0 {
                //         break 'outer '.';
                //     } else if val < 10 {
                //         break 'outer (val+48) as char;
                //     } else {
                //         break 'outer (val+55) as char;
                //     }
                // };
                // print!("{}", ch);
                if val > 0 {
                    occupied += 1;
                }
            }
            // println!();
        }

        it += 1;
        if occupied == 500 {
            //println!("Elapsed seconds: {it}\tOccupancy: {occupied}");
            secs = it;
            break;
        }
    }

    Some(secs)
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
