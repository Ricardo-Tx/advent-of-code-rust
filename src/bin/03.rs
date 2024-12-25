advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let template : &[u8]  = b"mul(_,_)";
    let mut i : usize = 0;
    let mut mult1 : u64 = 0;
    let mut mult2 : u64 = 0;
    let mut total : u64 = 0;

    let quit = |mult1: &mut u64, mult2: &mut u64, i: &mut usize| {
        *mult1 = 0;
        *mult2 = 0;
        *i = 0;
    };

    for b in input.bytes() {
        if b == template[i] {
            i += 1;
        } else {
            if i == 4 {
                if (mult1 == 0 && b >= b'1' && b <= b'9') || (mult1 > 0 && b >= b'0' && b <= b'9') {
                    mult1 *= 10;
                    mult1 += (b-48) as u64;
                } else if b == b',' {
                    i = 6;
                } else {                    
                    quit(&mut mult1, &mut mult2, &mut i);
                }
            } else if i == 6 {
                if (mult2 == 0 && b >= b'1' && b <= b'9') || (mult2 > 0 && b >= b'0' && b <= b'9') {
                    mult2 *= 10;
                    mult2 += (b-48) as u64;
                } else {
                    if b == b')' {
                        // total += 1;
                        // println!("{}: {}*{} = {}", total, mult1, mult2, mult1*mult2);
                        total += mult1*mult2;
                    } 
                    quit(&mut mult1, &mut mult2, &mut i);
                }
            } else {
                quit(&mut mult1, &mut mult2, &mut i);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let template : &[u8]  = b"mul(_,_)";
    let temp_do : &[u8]  = b"do()";
    let temp_dont : &[u8]  = b"don't()";
    let mut enable = true;

    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut k : usize = 0;
    let mut mult1 : u64 = 0;
    let mut mult2 : u64 = 0;
    let mut total : u64 = 0;
    
    let quit = |
        mult1: &mut u64, mult2: &mut u64, v: &mut usize| 
    {
        *mult1 = 0;
        *mult2 = 0;
        *v = 0;
    };
    
    for b in input.bytes() {
        // do template
        if b == temp_do[j] {
            j += 1;
            if j == temp_do.len() {
                enable = true;
                j = 0;
            }
        } else {
            j = 0;
        }
        
        // dont template
        if b == temp_dont[k] {
            k += 1;
            if k == temp_dont.len() {
                enable = false;
                k = 0;
            }
        } else {
            k = 0;
        }

        
        if !enable {
            continue;
        }

        // mul template
        if b == template[i] {
            i += 1;
        } else {
            if i == 4 {
                if (mult1 == 0 && b >= b'1' && b <= b'9') || (mult1 > 0 && b >= b'0' && b <= b'9') {
                    mult1 *= 10;
                    mult1 += (b-48) as u64;
                } else if b == b',' {
                    i = 6;
                } else {                    
                    quit(&mut mult1, &mut mult2, &mut i);
                }
            } else if i == 6 {
                if (mult2 == 0 && b >= b'1' && b <= b'9') || (mult2 > 0 && b >= b'0' && b <= b'9') {
                    mult2 *= 10;
                    mult2 += (b-48) as u64;
                } else {
                    if b == b')' {
                        // total += 1;
                        // println!("{}: {}*{} = {}", total, mult1, mult2, mult1*mult2);
                        total += mult1*mult2;
                    } 
                    quit(&mut mult1, &mut mult2, &mut i);
                }
            } else {
                quit(&mut mult1, &mut mult2, &mut i);
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
