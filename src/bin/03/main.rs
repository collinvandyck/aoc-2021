#![allow(unused)]
use core::panic;

fn main() {
    println!("pt1: {}", power_consumption(include_str!("in01")));
    println!("pt2: {}", life_support(include_str!("in01")));
}

fn power_consumption(s: &str) -> usize {
    let lines = s
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = lines[0].len();
    let cols = lines.len();
    let (gamma, epsilon) = (0..width).fold((0, 0), |(gamma, epsilon), col| {
        let chars = lines.iter().map(|l| l[width - 1 - col]);
        let c0 = chars.clone().filter(|c| *c == '0').count();
        let c1 = cols - c0;
        let gam_bit: usize = (c1 > c0).then_some(1).unwrap_or(0);
        let eps_bit: usize = (c1 > c0).then_some(0).unwrap_or(1);
        (gamma | (gam_bit << col), epsilon | (eps_bit << col))
    });
    gamma * epsilon
}

fn life_support(s: &str) -> usize {
    let lines = s
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let o2_rating = rating(&lines, |ones, zeroes| {
        (ones >= zeroes).then_some('1').unwrap_or('0')
    });
    let co2_rating = rating(&lines, |ones, zeroes| {
        (zeroes <= ones).then_some('0').unwrap_or('1')
    });
    o2_rating * co2_rating
}

fn rating(lines: &[Vec<char>], f: impl Fn(usize, usize) -> char) -> usize {
    let lines = lines.into_iter().map(|l| l.as_slice()).collect::<Vec<_>>();
    let cols = lines[0].len();
    let lines = (0..cols).fold(lines, |mut lines, col| {
        if lines.len() > 1 {
            let ones = lines.iter().filter(|chs| chs[col] == '1').count();
            let zeroes = lines.len() - ones;
            let keep = f(ones, zeroes);
            lines.retain(|chs| chs[col] == keep);
        }
        lines
    });
    assert_eq!(lines.len(), 1);
    let res = lines[0].iter().rev().enumerate().fold(0, |acc, (idx, ch)| {
        let b = (ch == &'1').then_some(1).unwrap_or(0);
        acc | (b << idx)
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt01() {
        assert_eq!(life_support(include_str!("in01")), 4406844);
    }

    #[test]
    fn ex01() {
        assert_eq!(power_consumption(include_str!("ex01")), 198);
    }

    #[test]
    fn ex02() {
        assert_eq!(life_support(include_str!("ex01")), 230);
    }
}
