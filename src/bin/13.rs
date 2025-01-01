advent_of_code::solution!(13);

#[derive(Debug)]
struct Prize {
    a: [i64;2],
    b: [i64;2],
    result: [i64;2],
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut prizes : Vec<Prize> = vec![
        Prize {
            a: [0, 0],
            b: [0, 0],
            result: [0, 0],
        }
    ];

    for (i, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            prizes.push(Prize {
                a: [0, 0],
                b: [0, 0],
                result: [0, 0],
            });
            continue;
        }
        let l = prizes.len();
        let end = match i%4 {
            0 => &mut prizes[l-1].a,
            1 => &mut prizes[l-1].b,
            2 => &mut prizes[l-1].result,
            _ => &mut prizes[l-1].a,
        };

        for b in line.bytes() {
            if b < b'0' || b > b'9' {
                if end[0] > 0 && end[1] < i64::MAX {
                    end[1] = i64::MAX;
                }
                continue;
            }

            if end[1] == 0 {                
                end[0] *= 10;
                end[0] += (b-48) as i64;
            } else {
                if end[1] == i64::MAX {
                    end[1] = 0;
                }
                end[1] *= 10;
                end[1] += (b-48) as i64;
            }
        }
    }
    prizes.pop();

    // println!("{:?}", prizes);

    let mut tokens : u64 = 0;
    for p in prizes {
        // AB to XY
        // | p.a[0] p.b[0] | 
        // | p.a[1] p.b[1] |

        // XY to AB
        // | p.b[1]  -p.b[0] | / det(AB to XY)
        // | -p.a[1]  p.a[0] |

        // let ab_to_xy = |v: [i64;2]| { 
        //     [p.a[0]*v[0]+p.b[0]*v[1], p.a[1]*v[0]+p.b[0]*v[1]] 
        // };
        
        let mut det = p.a[0]*p.b[1]-p.a[1]*p.b[0];
        let sign = det.signum();
        let xy_to_ab_det = |v: [i64;2]| { 
            [(p.b[1]*v[0]-p.b[0]*v[1])*sign, (p.a[0]*v[1]-p.a[1]*v[0])*sign]
        };
        det *= sign;

        if det != 0 {
            // println!("Buttons: {:?}, det {}", xy_to_ab_det(p.result), det);
            let mut presses = xy_to_ab_det(p.result);
            if presses[0] % det != 0 || presses[1] % det != 0 {
                continue;
            } else {
                presses[0] /= det;
                presses[1] /= det;
                tokens += (presses[0]*3 + presses[1]) as u64;
            }
        }
    }

    println!("{}", u64::MAX);
    Some(tokens)
}

pub fn part_two(input: &str) -> Option<u64> {
    // 10000000000000
    // 18446744073709551615

    let mut prizes : Vec<Prize> = vec![
        Prize {
            a: [0, 0],
            b: [0, 0],
            result: [0, 0],
        }
    ];

    for (i, line) in input.split('\n').enumerate() {
        if line.is_empty() {
            prizes.push(Prize {
                a: [0, 0],
                b: [0, 0],
                result: [0, 0],
            });
            continue;
        }
        let l = prizes.len();
        let end = match i%4 {
            0 => &mut prizes[l-1].a,
            1 => &mut prizes[l-1].b,
            2 => &mut prizes[l-1].result,
            _ => &mut prizes[l-1].a,
        };

        for b in line.bytes() {
            if b < b'0' || b > b'9' {
                if end[0] > 0 && end[1] < i64::MAX {
                    end[1] = i64::MAX;
                }
                continue;
            }

            if end[1] == 0 {                
                end[0] *= 10;
                end[0] += (b-48) as i64;
            } else {
                if end[1] == i64::MAX {
                    end[1] = 0;
                }
                end[1] *= 10;
                end[1] += (b-48) as i64;
            }
        }
        if i%4 == 2 {
            end[0] += 10000000000000;
            end[1] += 10000000000000;
        }
    }
    prizes.pop();

    // println!("{:?}", prizes);

    let mut tokens : u64 = 0;
    for p in prizes {
        // AB to XY
        // | p.a[0] p.b[0] | 
        // | p.a[1] p.b[1] |

        // XY to AB
        // | p.b[1]  -p.b[0] | / det(AB to XY)
        // | -p.a[1]  p.a[0] |

        // let ab_to_xy = |v: [i64;2]| { 
        //     [p.a[0]*v[0]+p.b[0]*v[1], p.a[1]*v[0]+p.b[0]*v[1]] 
        // };
        
        let mut det = p.a[0]*p.b[1]-p.a[1]*p.b[0];
        let sign = det.signum();
        let xy_to_ab_det = |v: [i64;2]| { 
            [(p.b[1]*v[0]-p.b[0]*v[1])*sign, (p.a[0]*v[1]-p.a[1]*v[0])*sign]
        };
        det *= sign;

        if det != 0 {
            // println!("Buttons: {:?}, det {}", xy_to_ab_det(p.result), det);
            let mut presses = xy_to_ab_det(p.result);
            if presses[0] % det != 0 || presses[1] % det != 0 {
                continue;
            } else {
                presses[0] /= det;
                presses[1] /= det;
                tokens += (presses[0]*3 + presses[1]) as u64;
            }
        }
    }

    Some(tokens)
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
