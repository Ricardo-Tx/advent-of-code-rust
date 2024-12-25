advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut safe : u64 = 0;
    for l in lines {
        if l.is_empty() {
            continue;
        }

        let mut last_num : Option<u64> = None; 
        let mut dir : Option<bool> = None;
        safe += 1;
        for n in l.split(' ') {
            let num : u64 = match n.parse() {
                Err(_) => continue,
                Ok(x) => x,
            };

            if last_num == None {
                last_num = Some(num);
                continue;
            }

            let delta = (num as i64) - (last_num.expect("It's fine") as i64);
            if delta < -3 || delta > 3 || delta == 0 {
                safe -= 1;
                break;
            }

            if dir == None {
                dir = Some(delta > 0);
                // continue;
            }

            if let Some(b) = dir {
                if (b && (delta < 1 || delta > 3)) || (!b && (delta > -1 || delta < -3)) {
                    safe -= 1;
                    break;
                }
            }

            last_num = Some(num);
        }
    }

    Some(safe)
}


fn valid(mut vector: Vec<i64>) -> bool {
    let mut sig : i64 = 0;
    for i in 0..vector.len()-1 {
        vector[i] = vector[i+1] - vector[i];
        if sig == 0 {
            sig = vector[i].signum()
        }
        vector[i] *= sig;
    }
    vector.pop();

    for e in vector {
        if e <= 0 || e > 3 {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');
    
    let mut safe : u64 = 0;
    for l in lines {
        if l.is_empty() {
            continue;
        }
    
        let mut nums : Vec<i64> = Vec::new();
        for n in l.split(' ') {
            let num : i64 = match n.parse() {
                Err(_) => continue,
                Ok(x) => x,
            };
            nums.push(num);
        }

        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            new_nums.remove(i); 

            if valid(new_nums) {
                safe += 1;
                break;
            }
        }

        // println!("{}, {:?}. {}", l, nums, cur_safe==safe);
        // println!("{}, {:?}", l, nums);
    }
    
    Some(safe)
    
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
