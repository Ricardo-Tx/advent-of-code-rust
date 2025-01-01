use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(19);

struct Node<'a> {
    char: u8,
    leaf : bool,
    children : HashMap<u8, &'a Node<'a>>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut root = Node {
        char: 0,
        leaf: false,
        children: HashMap::new(),
    };


    let mut patterns : HashSet<&str> = HashSet::new();
    let mut largest_pat : usize = 0;
    let mut possible : u64 = 0;
    for line in input.split('\n') {
        if patterns.is_empty() {
            for p in line.split(", ") {
                largest_pat = largest_pat.max(p.len());
                patterns.insert(p);
            }
            // println!("Patterns: {:?}\n", patterns);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        // println!("> {line}");
        // possible += ways_to_split(line, &patterns, largest_pat);
        // println!("{possible}\n");

        let mut designs : VecDeque<&str> = VecDeque::from([line]);
        // println!("Checking {line}:");
        while !designs.is_empty() {
            // println!("{:?}", designs);
            let cur_design = designs.pop_front()?;
            if patterns.contains(cur_design) {
                possible += 1;
                // println!("DONE");
                break;
            }
            for i in 1..cur_design.len().min(largest_pat+1) {
                if patterns.contains(&cur_design[..i]) {
                    let new_design = &cur_design[i..];
                    if !designs.contains(&new_design) {
                        designs.push_back(new_design);
                        // println!(" - added {:?}", new_design);
                    }
                }
            }
        }
        // println!();
    }

    Some(possible)
}

// 47314 is too low
pub fn part_two(input: &str) -> Option<u64> {
    let mut patterns : HashSet<&str> = HashSet::new();
    let mut largest_pat : usize = 0;
    let mut possible : u64 = 0;
    for line in input.split('\n') {
        if patterns.is_empty() {
            for p in line.split(", ") {
                largest_pat = largest_pat.max(p.len());
                patterns.insert(p);
            }
            // println!("Patterns: {:?}\n", patterns);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let mut designs : VecDeque<Vec<usize>> = VecDeque::from([Vec::from([])]);
        // println!("Checking {line}:");
        while !designs.is_empty() {
            // println!("{:?}", designs);
            let mut cur_design = designs.pop_front()?;
            let back = if cur_design.is_empty() { None } else { Some(cur_design[cur_design.len()-1]) };
            if let Some(b) = back {
                if patterns.contains(&line[b..]) {
                    possible += 1;
                    // println!("AHHHHH {possible}");
                }
                // let mut print_design = cur_design.clone();
                // print_design.push(back);
                // println!(" ! found pattern {:?}", print_delsign);
            }

            let l = match back {
                Some(x) => x,
                None => 0,
            };
            for i in l+1..line.len().min(l+largest_pat+1) {
                let piece_back = &line[l..i];
                // let piece_front = &line[i..];
                if patterns.contains(piece_back) {
                    cur_design.push(i);
                    designs.push_back(cur_design.clone());
                    cur_design.pop();
                }
            }
        }
        // println!();
    }

    Some(possible)
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
