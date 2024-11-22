#![allow(unused)]

fn main() {
    println!("pt1: {}", pilot(include_str!("in01")));
    println!("pt2: {}", pilot_aim(include_str!("in01")));
}

fn pilot_aim(s: &str) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for mv in parse(s) {
        match mv {
            Move::Forward(n) => {
                pos += n;
                depth += aim * n;
            }
            Move::Down(n) => aim += n,
            Move::Up(n) => aim -= n,
        }
    }
    pos * depth
}

fn pilot(s: &str) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    for mv in parse(s) {
        match mv {
            Move::Forward(n) => pos += n,
            Move::Down(n) => depth += n,
            Move::Up(n) => depth -= n,
        }
    }
    pos * depth
}

fn parse(s: &str) -> impl Iterator<Item = Move> + '_ {
    s.trim()
        .lines()
        .map(|l| l.trim().split_once(" ").unwrap())
        .map(|(dir, n)| {
            let n = n.parse::<i64>().unwrap();
            match dir {
                "forward" => Move::Forward(n),
                "down" => Move::Down(n),
                "up" => Move::Up(n),
                _ => panic!("bad dir: {dir}"),
            }
        })
}

enum Move {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        assert_eq!(pilot(include_str!("ex01")), 150);
    }

    #[test]
    fn ex02() {
        assert_eq!(pilot_aim(include_str!("ex01")), 900);
    }
}
