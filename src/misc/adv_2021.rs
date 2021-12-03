use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn day_01_1() -> Result<i32, Error> {
    let mut nums = BufReader::new(File::open("assets/adv_2021/input01.txt")?)
        .lines()
        .map(|num_str| num_str.unwrap().parse::<i32>().unwrap());

    let mut prev = nums.next().unwrap();
    let mut count = 0;

    for curr in nums {
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }

    Ok(count)
}

fn day_01_2() -> Result<i32, Error> {
    let nums = BufReader::new(File::open("assets/adv_2021/input01.txt")?)
        .lines()
        .map(|num_str| num_str.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut prev = nums[..3].iter().sum::<i32>();
    let mut count = 0;

    for (idx, num) in nums[3..].iter().enumerate() {
        let curr = prev - nums[idx] + num;
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }

    Ok(count)
}

fn day_02_1() -> Result<i32, Error> {
    let hd = BufReader::new(File::open("assets/adv_2021/input02.txt")?)
        .lines()
        .fold((0, 0), |(h, d), action| {
            let action = action.unwrap();
            match &action[..2] {
                "up" => (h, d - action[3..].parse::<i32>().unwrap()),
                "do" => (h, d + action[5..].parse::<i32>().unwrap()),
                "fo" => (h + action[8..].parse::<i32>().unwrap(), d),
                _ => (h, d),
            }
        });
    Ok(hd.0 * hd.1)
}

fn day_02_2() -> Result<i32, Error> {
    let hd = BufReader::new(File::open("assets/adv_2021/input02.txt")?)
        .lines()
        .fold((0, 0, 0), |(h, a, d), action| {
            let action = action.unwrap();
            match &action[..2] {
                "fo" => {
                    let v = action[8..].parse::<i32>().unwrap();
                    (h + v, a, d + a * v)
                }
                "up" => (h, a - action[3..].parse::<i32>().unwrap(), d),
                "do" => (h, a + action[5..].parse::<i32>().unwrap(), d),
                _ => (h, a, d),
            }
        });
    Ok(hd.0 * hd.2)
}

fn day_03_1() -> Result<i32, Error> {
    let e = BufReader::new(File::open("assets/adv_2021/input03.txt")?)
        .lines()
        .fold(vec![0; 12], |mut freq, bin| {
            for (i, ch) in bin.unwrap().chars().enumerate() {
                if ch == '1' {
                    freq[i] += 1;
                } else {
                    freq[i] -= 1;
                }
            }
            freq
        })
        .into_iter()
        .fold(0, |e, v| (e << 1) + ((v >> 31) & 1));
    Ok(e * (!e & 0xfff))
}

fn day_03_2() -> Result<i32, Error> {
    let lines = BufReader::new(File::open("assets/adv_2021/input03.txt")?)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let find = |mut bins: Vec<Vec<char>>, most: bool| {
        for i in 0..12 {
            let (l, r): (Vec<Vec<char>>, Vec<Vec<char>>) =
                bins.into_iter().partition(|chars| chars[i] == '1');
            bins = match l.len().cmp(&r.len()) {
                Ordering::Equal => {
                    if most {
                        l
                    } else {
                        r
                    }
                }
                Ordering::Less => {
                    if most {
                        r
                    } else {
                        l
                    }
                }
                Ordering::Greater => {
                    if most {
                        l
                    } else {
                        r
                    }
                }
            };
            if bins.len() == 1 {
                break;
            }
        }
        i32::from_str_radix(&bins[0].iter().collect::<String>(), 2).unwrap()
    };

    let o = find(lines.clone(), true);
    let c = find(lines, false);

    Ok(o * c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_1() {
        assert_eq!(1616, day_01_1().unwrap());
    }

    #[test]
    fn test_day_01_2() {
        assert_eq!(1645, day_01_2().unwrap());
    }

    #[test]
    fn test_day_02_1() {
        assert_eq!(2117664, day_02_1().unwrap());
    }

    #[test]
    fn test_day_02_2() {
        assert_eq!(2073416724, day_02_2().unwrap());
    }

    #[test]
    fn test_day_03_1() {
        assert_eq!(1458194, day_03_1().unwrap())
    }

    #[test]
    fn test_day_03_2() {
        assert_eq!(2829354, day_03_2().unwrap())
    }
}
