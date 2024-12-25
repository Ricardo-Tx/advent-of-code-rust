advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut list1 : Vec<u64> = Vec::new();
    let mut list2 : Vec<u64> = Vec::new();
    let mut total : u64 = 0;

    for l in lines {
        if l.len() == 0 {
            continue
        }
        let s = String::from(l);

        list1.push(match (&s[0..5]).parse() {
            Ok(x) => x,
            Err(_) => 0,
        });
        list2.push(match (&s[8..13]).parse() {
            Ok(x) => x,
            Err(_) => 0,
        });
    }
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        total += list1[i].abs_diff(list2[i]);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut list1 : Vec<u64> = Vec::new();
    let mut list2 : Vec<u64> = Vec::new();
    let mut sim : u64 = 0;

    for l in lines {
        if l.len() == 0 {
            continue
        }
        let s = String::from(l);

        list1.push(match (&s[0..5]).parse() {
            Ok(x) => x,
            Err(_) => 0,
        });
        list2.push(match (&s[8..13]).parse() {
            Ok(x) => x,
            Err(_) => 0,
        });
    }
    list1.sort();
    list2.sort();

    let mut i : usize = 0;
    for num in list1 {
        let mut k : u64 = 0;
        while list2[i] <= num {
            if list2[i] == num {
                k += 1;
            }
            i += 1;
        }
        sim += num * k;
    }

    Some(sim)
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
