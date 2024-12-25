use std::{collections::{HashMap, HashSet}, usize};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.split('\n');
    let mut map : HashMap<u8, HashSet<u8>> = HashMap::new();

    loop {
        let line = it.next().expect("aahahhh");
        if line.is_empty() {
            break;
        }
        
        let key : u8 = line[0..2].parse().expect("grrrrr");
        let value : u8 = line[3..5].parse().expect("grrrrr");

        if map.contains_key(&key) {
            map.get_mut(&key).expect("hhhhhhhh").insert(value);
        } else {
            map.insert(key, HashSet::from([value]));
        }
    }
    
    let mut total : u64 = 0;
    'outer: loop {
        let line = it.next().expect("aahahhh");
        if line.is_empty() {
            break;
        }

        let mut pages : HashSet<u8> = HashSet::new();
        for num in line.split(',') {
            let n : u8 = num.parse().expect("grrrrr");

            if pages.len() > 0 && map.contains_key(&n) {
                if let Some(_) = map.get(&n).expect("jjjjj").intersection(&pages).next() {
                    continue 'outer;
                }
            }
            pages.insert(n);
        }

        let pos : usize = (line.len()+1)/6;
        let middle : u8 = (&String::from(line)[pos*3..pos*3+2]).parse().expect("hahahhaha");
        total += middle as u64;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut it = input.split('\n');
    let mut map : HashMap<u8, HashSet<u8>> = HashMap::new();
    
    loop {
        let line = it.next().expect("aahahhh");
        if line.is_empty() {
            break;
        }
        
        let key : u8 = line[0..2].parse().expect("grrrrr");
        let value : u8 = line[3..5].parse().expect("grrrrr");
    
        if map.contains_key(&key) {
            map.get_mut(&key).expect("hhhhhhhh").insert(value);
        } else {
            map.insert(key, HashSet::from([value]));
        }
    }
    
    let mut total : u64 = 0;
    loop {
        let line = it.next().expect("aahahhh");
        if line.is_empty() {
            break;
        }
        println!("{line}");

        let mut pages : Vec<u8> = Vec::new();
        for num in line.split(',') {
            let n : u8 = num.parse().expect("grrrrr");
            pages.push(n);
        }
        
        // sorting
        let mut index = pages.len()-1;
        let mut ordered : bool = true;
        while index > 0 {
            let last = pages.remove(index);
            println!("Removed at index {}", index);
            let hashmap = map.get(&last);
            if hashmap == None {                
                pages.insert(index, last);
                index -= 1;
                continue;
            }
            let hashmap = hashmap.expect("h");

            let swapped : bool = 'outer: {
                for i in 0..index {
                    if hashmap.contains(&pages[i]) {
                        pages.insert(i, last);
                        println!("Moved {last} to index {i}");
                        println!("{pages:?}");
                        break 'outer true;
                    } else {
                        println!("No rule {}|{}", last, pages[i]);
                    }
                }
                false
            };

            if !swapped {
                pages.insert(index,last);
                index -= 1;
            } else {
                ordered = false;
            }
        }

    
        // let pos : usize = (line.len()+1)/6;
        // let middle : u8 = (&String::from(line)[pos*3..pos*3+2]).parse().expect("hahahhaha");
        // total += middle as u64;
        // total += 1
        // println!();

        if !ordered {
            total += pages[pages.len()/2] as u64;
            // total += 1;
            // println!("{pages:?}");
        }
        println!();
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
