#![allow(unused)]
use ansi_term::Style;
use core::panic;
use std::{borrow::BorrowMut, fmt::Display};

fn main() {
    println!("pt1: {}", parse_game(include_str!("in01")).first_win());
    println!("pt2: {}", parse_game(include_str!("in01")).last_win());
}

struct Game {
    nums: Vec<usize>,
    boards: Vec<Board>,
}

impl Game {
    fn first_win(&mut self) -> usize {
        for num in self.nums.iter().copied() {
            for board in self.boards.iter_mut() {
                if board.mark(num) {
                    return board.score(num);
                }
            }
        }
        panic!("no board won");
    }
    fn last_win(&mut self) -> usize {
        let mut last: Option<usize> = None;
        for num in self.nums.iter().copied() {
            for board in self.boards.iter_mut().filter(|b| !b.won) {
                if board.mark(num) {
                    last.replace(board.score(num));
                }
            }
        }
        last.expect("no last board won")
    }
}

const DIM: usize = 5;

#[derive(Clone)]
struct Board {
    tiles: Vec<Tile>,
    won: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .chunks(DIM)
            .map(|t| t.iter().map(ToString::to_string).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{s}")
    }
}

impl Board {
    fn new(tiles: Vec<Tile>) -> Self {
        Self { tiles, won: false }
    }
    fn score(&self, num: usize) -> usize {
        let sum: usize = self
            .tiles
            .iter()
            .filter_map(|t| (!t.hit).then_some(t.num))
            .sum();
        sum * num
    }
    fn mark(&mut self, num: usize) -> bool {
        if let Some(Tile { num, hit }) = self.tiles.iter_mut().find(|t| t.num == num) {
            *hit = true;
            if self.win() {
                self.won = true;
                return true;
            }
        }
        false
    }
    fn win(&self) -> bool {
        for n in 0..DIM {
            if self.row(n).all(|t| t.hit) {
                return true;
            }
            if self.col(n).all(|t| t.hit) {
                return true;
            }
        }
        false
    }
    fn col(&self, col: usize) -> impl Iterator<Item = &Tile> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(idx, t)| ((idx + DIM) % DIM == col).then_some(t))
    }
    fn row(&self, n: usize) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().skip(DIM * n).take(DIM)
    }
    fn get(&self, row: usize, col: usize) -> &Tile {
        &self.tiles[row * DIM + col]
    }
}

#[derive(Clone, Copy)]
struct Tile {
    num: usize,
    hit: bool,
}

impl Tile {
    fn new(num: usize) -> Self {
        Self { num, hit: false }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}", self.num);
        if self.hit {
            s = ansi_term::Color::Cyan
                .bold()
                .underline()
                .paint(s)
                .to_string();
        }
        if self.num < 10 {
            s.push(' ');
        }
        s.push(' ');
        write!(f, "{s}")
    }
}

fn parse_game(s: &str) -> Game {
    let mut lines = s.trim().lines().map(str::trim);
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(lines.next().unwrap(), "");
    let mut boards = vec![];
    loop {
        let tiles = lines
            .borrow_mut()
            .take_while(|l| !l.is_empty())
            .flat_map(str::split_whitespace)
            .map(str::parse)
            .map(Result::unwrap)
            .map(Tile::new)
            .collect::<Vec<_>>();
        if tiles.is_empty() {
            break;
        }
        assert_eq!(tiles.len(), 25);
        boards.push(Board::new(tiles));
    }
    Game { nums, boards }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01() {
        let mut game = super::parse_game(include_str!("ex01"));
        let score = game.first_win();
        assert_eq!(score, 4512);
    }

    #[test]
    fn expt02() {
        let mut game = super::parse_game(include_str!("ex01"));
        let score = game.last_win();
        assert_eq!(score, 1924);
    }

    #[test]
    fn parse() {
        let game = super::parse_game(include_str!("ex01"));
        assert_eq!(3, game.boards.len());
    }
}
