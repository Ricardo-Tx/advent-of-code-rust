use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn hash(n : &mut u64) {
    *n ^= *n << 6;
    *n %= 16777216;

    *n ^= *n >> 5;
    *n %= 16777216;

    *n ^= *n << 11;
    *n %= 16777216;
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut secrets : u64 = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut n : u64 = line.parse().expect("yuh");
        for _ in 0..2000 {
            hash(&mut n);
        }
        // println!("{line}: {n}");
        secrets += n;
    }

    Some(secrets)
}

// 1578 is too high
pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys : Vec<HashMap<u32,u8>> = Vec::new();
    let mut seqs : HashSet<u32> = HashSet::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut n : u64 = line.parse().expect("yuh");

        let mut monkey : HashMap<u32,u8> = HashMap::new();
        let mut last_price : u8 = 255;
        let mut record : u32 = 0;
        for i in 0u16..2000u16 {
            hash(&mut n);
            let price : u8 = (n % 10) as u8;
            let diff : i16 = price as i16 - last_price as i16;
            record <<= 5;
            record |= (diff+9) as u32 & 31;

            if i > 3 {
                let key = record & 1048575;
                if let Some(val) = monkey.get_mut(&key) {
                    *val = price.max(*val);
                } else {
                    monkey.insert(key, price);
                    seqs.insert(key);
                }
            }

            last_price = price;
        }

        // println!("{:?}", monkey.get(&239884));
        monkeys.push(monkey);
    }
    // println!("{:?}", seqs.len());
    let mut best_seq : u32 = 0; 
    let mut best_total : u64 = 0;
    for seq in seqs.iter() {
        let mut total : u64 = 0;
        for m in monkeys.iter() {
            if let Some(price) = m.get(seq) {
                total += *price as u64;
            }
        }
        if total > best_total {
            best_total = total;
            best_seq = *seq;
        }
    }

    println!("Best price: {best_total}\tBest sequence: {},{},{},{}", 
        (best_seq >> 15) as i32 - 9, 
        ((best_seq >> 10) & 31) as i32 - 9, 
        ((best_seq >> 5) & 31) as i32 - 9, 
        (best_seq & 31) as i32 - 9
    );
    
    Some(best_total)
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
