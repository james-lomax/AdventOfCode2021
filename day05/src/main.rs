#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use vector2d::Vector2D;
use regex::Regex;
use itertools::Itertools;

type Vec2i = Vector2D<i32>;

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Line(Vec2i, Vec2i);

impl Line {
    fn is_axis_aligned(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    // Note that this only works for axis aligned lines
    // Others return None
    fn aa_direction(&self) -> Option<Vec2i> {
        let diff = self.1 - self.0;
        if diff.x == 0 {
            Some(Vec2i::new(0, diff.y / diff.y.abs()))
        } else if diff.y == 0 {
            Some(Vec2i::new(diff.x / diff.x.abs(), 0))
        } else {
            None
        }
    }

    fn trace(&self, touched: &mut HashMap<(i32, i32), usize>) {
        let dir = self.aa_direction().expect("Trace requires axis-aligned lines");
        let mut cpos = self.0;
        while cpos != self.1 {
            *touched.entry(cpos.into()).or_insert(0) += 1;
            cpos += dir;
        }
        *touched.entry(cpos.into()).or_insert(0) += 1;
    }
}


fn parse_lines(contents: &str) -> Vec<Line> {
    contents.split('\n')
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|line| {
            NUMBER_PATTERN.find_iter(line)
                .map(|num_s| num_s.as_str().parse::<i32>().unwrap())
                .next_tuple::<(i32, i32, i32, i32)>()
                .expect("Syntax error") 
        })
        .map(|(x1, y1, x2, y2)| Line(Vec2i::new(x1, y1), Vec2i::new(x2, y2)))
        .collect()
}

fn p1_overlap_aa_lines(lines: &Vec<Line>) -> usize {
    let mut touched = HashMap::<(i32, i32), usize>::new();
    for line in lines {
        if line.is_axis_aligned() {
            line.trace(&mut touched);
        }
    }
    touched.values().filter(|v| **v > 1).count()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    let lines = parse_lines(&contents);
    println!("Part 1 = {}", p1_overlap_aa_lines(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";
        
        let lines = parse_lines(sample);
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], Line(Vec2i::new(0, 9), Vec2i::new(5, 9)));
        assert_eq!(lines[9], Line(Vec2i::new(5, 5), Vec2i::new(8, 2)));

        assert_eq!(p1_overlap_aa_lines(&lines), 5);
    }
}