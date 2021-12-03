use std::{
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
}
