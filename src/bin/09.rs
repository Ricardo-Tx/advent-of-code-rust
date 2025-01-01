advent_of_code::solution!(9);

// 00...111...2...333.44.5555.6666.777.888899
// 009..111...2...333.44.5555.6666.777.88889.
// 0099.111...2...333.44.5555.6666.777.8888..
// 00998111...2...333.44.5555.6666.777.888...
// 009981118..2...333.44.5555.6666.777.88....
// 0099811188.2...333.44.5555.6666.777.8.....
// 009981118882...333.44.5555.6666.777.......
// 0099811188827..333.44.5555.6666.77........
// 00998111888277.333.44.5555.6666.7.........
// 009981118882777333.44.5555.6666...........
// 009981118882777333644.5555.666............
// 00998111888277733364465555.66.............
// 0099811188827773336446555566..............

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk : Vec<u64> = Vec::new();
    for b in input[0..input.len()-1].bytes() {
        disk.push((b-48) as u64);
    }

    let mut index : usize = 0;
    let mut cursor : u64 = 0;
    let mut checksum : u64 = 0;
    loop {
        if index >= disk.len() {
            break;
        }
        let is_file = index % 2 == 0; 

        if is_file {
            // checksum formula
            let size = disk[index];
            // println!("FILE : file index {}. size {size}. cursor {cursor}. added {} to checksum, cursor now at {}", index/2, (index/2) as u64 * (cursor*2+size-1)*size/2, cursor+size);
            checksum += (index/2) as u64 * (cursor*2+size-1)*size/2;
            cursor += size;
        } else {
            let mut space = disk[index];
            while space > 0 {
                let last_index = disk.len()-1;
                let size = space.min(disk[last_index]);
                
                disk[last_index] -= size;
                disk[index] -= size;
                space -= size;
                
                if disk[last_index] == 0 {
                    disk.pop();
                    disk.pop();
                }
                
                // println!("EMPTY: file index {}. size {size}. cursor {cursor}. added {} to checksum, cursor now at {}", last_index/2, (last_index/2) as u64 * (cursor*2+size-1)*size/2, cursor+size);
                checksum += (last_index/2) as u64 * (cursor*2+size-1)*size/2;
                cursor += size;
            }
        }

        index += 1;
    }

    Some(checksum)
}

// 00...111...2...333.44.5555.6666.777.888899
// 0099.111...2...333.44.5555.6666.777.8888..
// 0099.1117772...333.44.5555.6666.....8888..
// 0099.111777244.333....5555.6666.....8888..
// 00992111777.44.333....5555.6666.....8888..

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk : Vec<u64> = Vec::new();
    for b in input[0..input.len()-1].bytes() {
        disk.push((b-48) as u64);
    }
    let start_len = disk.len() as u64;

    let mut index : usize = 0;
    let mut cursor : u64 = 0;
    let mut checksum : u64 = 0;
    loop {
        if index >= disk.len() {
            break;
        }
        let is_file = index % 2 == 0; 

        if is_file {
            // checksum formula
            let size = disk[index];
            if size < start_len {
                // println!("FILE : file index {}. size {size}. cursor {cursor}. added {} to checksum, cursor now at {}", index/2, (index/2) as u64 * (cursor*2+size-1)*size/2, cursor+size);
                checksum += (index/2) as u64 * (cursor*2+size-1)*size/2;
                cursor += size;
            } else {
                cursor += size - start_len;
            }
        } else {
            let mut space = disk[index];
            while space > 0 {
                // find a file that fits here
                let fit_index_opt = 'outer: {
                    for i in (index+1..disk.len()).step_by(2).rev() {
                        if disk[i] <= space {
                            break 'outer Some(i);
                        }
                    }
                    None
                };


                if let Some(fit_index) = fit_index_opt {
                    let size = disk[fit_index];
                    disk[index] -= size;
                    space -= size;

                    disk[fit_index] += start_len;

                    // println!("EMPTY: file index {}. size {size}. cursor {cursor}. added {} to checksum, cursor now at {}", fit_index/2, (fit_index/2) as u64 * (cursor*2+size-1)*size/2, cursor+size);
                    checksum += (fit_index/2) as u64 * (cursor*2+size-1)*size/2;
                    cursor += size;
                } else {
                    cursor += space;
                    break;
                }
            }
        }

        index += 1;
    }

    Some(checksum)
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
