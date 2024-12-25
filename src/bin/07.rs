advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total : u64 = 0;
    for line in input.split('\n') {
        let mut res : u64 = 0;
        let mut operands : Vec<u64> = Vec::new();
        if line.is_empty() {
            continue;
        }

        for num in line.split(' ') {
            if res == 0 {
                res = num[0..num.len()-1].parse().expect("grr");
            } else {
                operands.push(num.parse().expect("jh b"));
            }
        }

        for i in 0..2u32.pow((operands.len()-1) as u32) {
            let mut val = operands[0];
            let mut b : u32 = 1;
            for j in 1..operands.len() {
                if i & b != 0 {
                    val += operands[j]
                } else {
                    val *= operands[j]
                }
                b <<= 1;
            }

            if val == res {
                total += res;
                break;
            }
        }

        // println!("{res}, {operands:?}");
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total : u64 = 0;
    for line in input.split('\n') {
        let mut res : u64 = 0;
        let mut operands : Vec<u64> = Vec::new();
        if line.is_empty() {
            continue;
        }

        for num in line.split(' ') {
            if res == 0 {
                res = num[0..num.len()-1].parse().expect("grr");
            } else {
                operands.push(num.parse().expect("jh b"));
            }
        }

        for i in 0..3u32.pow((operands.len()-1) as u32) {
            let mut val = operands[0];
            let mut b : u32 = 1;

            for j in 1..operands.len() {
                match (i / b) % 3 {
                    0 => val += operands[j],
                    1 => val *= operands[j],
                    2 => {
                        let digits = format!("{}", operands[j]).len() as u32;
                        val = val * (10u32.pow(digits) as u64) + operands[j];
                    },
                    _ => {}
                }
                b *= 3;
            }

            if val == res {
                total += res;
                break;
            }
        }

        // println!("{res}, {operands:?}");
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
