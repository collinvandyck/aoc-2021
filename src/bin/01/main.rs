#![allow(unused)]

fn main() {
    println!("pt1: {}", count(include_str!("in01")));
    println!("pt2: {}", count_sliding(include_str!("in01")));
}

fn count(s: &str) -> usize {
    let nums = s
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.parse::<u32>().unwrap());
    nums.clone().zip(nums.skip(1)).fold(0, acc_if_greater)
}

fn count_sliding(s: &str) -> usize {
    let nums = s
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let nums = nums.windows(3).map(|f| f.iter().sum::<u32>());
    nums.clone().zip(nums.skip(1)).fold(0, acc_if_greater)
}

fn acc_if_greater(acc: usize, (a, b): (u32, u32)) -> usize {
    acc + (b > a).then_some(1).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        assert_eq!(count(include_str!("ex01")), 7);
    }

    #[test]
    fn ex01sliding() {
        assert_eq!(count_sliding(include_str!("ex01")), 5);
    }
}
