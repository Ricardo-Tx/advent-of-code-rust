advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        grid.push(line.as_bytes().to_vec());
    }
    
    let mut h_words : u64 = 0;
    let mut h_i_words : u64 = 0;
    let mut v_words : u64 = 0;
    let mut v_i_words : u64 = 0;
    let mut pd_words : u64 = 0;
    let mut pd_i_words : u64 = 0;
    let mut nd_words : u64 = 0;
    let mut nd_i_words : u64 = 0;
    for y in 0..grid.len() {        
        for x in 0..grid[0].len() {
            // horizontal
            if x < grid[0].len()-3 {
                if grid[y][x] == b'X' && grid[y][x+1] == b'M' && grid[y][x+2] == b'A' && grid[y][x+3] == b'S' {
                    h_words += 1;
                } else if grid[y][x] == b'S' && grid[y][x+1] == b'A' && grid[y][x+2] == b'M' && grid[y][x+3] == b'X' {
                    h_i_words += 1;
                }
            }
            
            // vertical
            if y < grid.len()-3 {
                if grid[y][x] == b'X' && grid[y+1][x] == b'M' && grid[y+2][x] == b'A' && grid[y+3][x] == b'S' {
                    v_words += 1;
                } else if grid[y][x] == b'S' && grid[y+1][x] == b'A' && grid[y+2][x] == b'M' && grid[y+3][x] == b'X' {
                    v_i_words += 1;
                }                
            }
            
            // positive diagonal
            if x < grid[0].len()-3 && y < grid.len()-3 {
                if grid[y][x] == b'X' && grid[y+1][x+1] == b'M' && grid[y+2][x+2] == b'A' && grid[y+3][x+3] == b'S' {
                    pd_words += 1;
                } else if grid[y][x] == b'S' && grid[y+1][x+1] == b'A' && grid[y+2][x+2] == b'M' && grid[y+3][x+3] == b'X' {
                    pd_i_words += 1;
                }                                
            }
            
            // negative diagonal
            if x > 2 && y < grid.len()-3 {
                if grid[y+3][x-3] == b'X' && grid[y+2][x-2] == b'M' && grid[y+1][x-1] == b'A' && grid[y][x] == b'S' {
                    nd_words += 1;
                } else if grid[y+3][x-3] == b'S' && grid[y+2][x-2] == b'A' && grid[y+1][x-1] == b'M' && grid[y][x] == b'X' {
                    nd_i_words += 1;
                }                                
            }
        }
    }

    // println!("{} {} {} {} {} {} {} {}", h_words, h_i_words, v_words, v_i_words, pd_words, pd_i_words, nd_words, nd_i_words);
    println!("{} {}", h_words, h_i_words);
    println!("{} {}", v_words, v_i_words);
    println!("{} {}", pd_words, pd_i_words);
    println!("{} {}", nd_words, nd_i_words);

    Some(h_words + h_i_words + v_words + v_i_words + pd_words + pd_i_words + nd_words + nd_i_words)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid : Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        grid.push(line.as_bytes().to_vec());
    }
    
    let mut crosses : u64 = 0;
    for y in 1..grid.len()-1 {        
        for x in 1..grid[0].len()-1 {
            if grid[y][x] != b'A' {
                continue;
            }

            let tl = grid[y-1][x-1];
            let tr = grid[y-1][x+1];
            let bl = grid[y+1][x-1];
            let br = grid[y+1][x+1];

            match tl {
                b'M' => {
                    if tr == b'M' {
                        crosses += (bl == b'S' && br == b'S') as u64;
                    } else if bl == b'M' {
                        crosses += (tr == b'S' && br == b'S') as u64;
                    }
                },
                b'S' => {
                    if tr == b'S' {
                        crosses += (bl == b'M' && br == b'M') as u64;
                    } else if bl == b'S' {
                        crosses += (tr == b'M' && br == b'M') as u64;
                    }
                }
                _ => continue,
            }
        }
    }
    
    Some(crosses)
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
