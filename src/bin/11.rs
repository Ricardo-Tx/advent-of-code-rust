use std::collections::HashMap;

advent_of_code::solution!(11);

// 0 -> 1
// 1 -> 2024 -> 20 24 -> 2 0 2 4
// 2 -> 4048 -> 40 48 -> 4 0 4 8
// 3 -> 6072 -> 60 72 -> 6 0 7 2
// 4 -> 8096 -> 80 96 -> 8 0 9 6
// 5 -> 10120 -> 20482880 -[x3]-> 2 0 4 8 2 8 2 0
// 6 -> 12144 -> 24579456 -[x3]-> 2 4 5 7 9 4 5 6
// 7 -> 14168 -> 28676032 -[x3]-> 2 8 6 7 6 0 3 2
// 8 -> 16192 -> 32772608 -[x3]-> 3 2 7 7 2 6 0 8
// 9 -> 18216 -> 36869184 -[x3]-> 3 6 8 6 9 1 8 4


// 383 is too low
pub fn part_one(input: &str) -> Option<u64> {
    let update_stones = |st : &mut HashMap<u64,u64>, k : u64, v : u64| {
        if let Some(val) = st.get_mut(&k) {
            *val += v;
        } else {
            st.insert(k, v);
        }
    };

    let mut stones : HashMap<u64,u64> = HashMap::new();
    for num in input.trim().split(' ') {
        let n : u64 = num.parse().expect("stone!");
        update_stones(&mut stones, n, 1);
    }   
    

    // println!("{stones:?}");
    for _ in 0..25 {
        let mut new_stones : HashMap<u64,u64> = HashMap::new();
        for (&num, &count) in stones.iter() {
            if num == 0 {
                // increment
                update_stones(&mut new_stones, 1, count);
            } else {
                let pow_ten_opt : Option<u64> = 'outer: {
                    let mut pow_ten : u64 = 10;
                    let mut res_ten : u64 = 10;
                    while pow_ten < u64::MAX / 100 {
                        if num >= pow_ten && num < pow_ten*10 {
                            break 'outer Some(res_ten);
                        }
                        pow_ten *= 100;
                        res_ten *= 10;
                    }
                    None
                };
                
                if let Some(ten) = pow_ten_opt {
                    update_stones(&mut new_stones, num/ten, count);        
                    update_stones(&mut new_stones, num%ten, count);        
                } else {
                    update_stones(&mut new_stones, num*2024, count);        
                }
            }
        }

        stones = new_stones;
        // println!("{stones:?}");
    }

    
    let mut total : u64 = 0;
    for c in stones.values() {
        total += c;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let update_stones = |st : &mut HashMap<u64,u64>, k : u64, v : u64| {
        if let Some(val) = st.get_mut(&k) {
            *val += v;
        } else {
            st.insert(k, v);
        }
    };

    let mut stones : HashMap<u64,u64> = HashMap::new();
    for num in input.trim().split(' ') {
        let n : u64 = num.parse().expect("stone!");
        update_stones(&mut stones, n, 1);
    }   
    

    // println!("{stones:?}");
    for _ in 0..75 {
        let mut new_stones : HashMap<u64,u64> = HashMap::new();
        for (&num, &count) in stones.iter() {
            if num == 0 {
                // increment
                update_stones(&mut new_stones, 1, count);
            } else {
                let pow_ten_opt : Option<u64> = 'outer: {
                    let mut pow_ten : u64 = 10;
                    let mut res_ten : u64 = 10;
                    while pow_ten < u64::MAX / 100 {
                        if num >= pow_ten && num < pow_ten*10 {
                            break 'outer Some(res_ten);
                        }
                        pow_ten *= 100;
                        res_ten *= 10;
                    }
                    None
                };
                
                if let Some(ten) = pow_ten_opt {
                    update_stones(&mut new_stones, num/ten, count);        
                    update_stones(&mut new_stones, num%ten, count);        
                } else {
                    update_stones(&mut new_stones, num*2024, count);        
                }
            }
        }

        stones = new_stones;
        // println!("{stones:?}");
    }

    
    let mut total : u64 = 0;
    for c in stones.values() {
        total += c;
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
