use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Clone)]
struct Coordinate {
    col: usize,
    row: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from((col, row): (usize, usize)) -> Self {
        Self { col, row }
    }
}

struct Edge {
    begin: Coordinate,
    end: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Edge {
    fn from((begin, end): (Coordinate, Coordinate)) -> Self {
        Self { begin, end }
    }
}

struct Image {
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
    paths: Vec<Edge>,
}

impl From<&str> for Image {
    fn from(input: &str) -> Self {
        // -- Parse input.
        let width = input.lines().next().unwrap().len();
        let mut galaxies = Vec::new();
        let mut col_counter = vec![0; width];
        let mut empty_rows = Vec::new();

        for (row, line) in input.lines().enumerate() {
            // -- Check if row is empty.
            if line.chars().all(|c| c == '.') {
                empty_rows.push(row);
                continue;
            }

            for (col, c) in line.char_indices() {
                if c == '#' {
                    let coord = Coordinate::from((col, row));
                    galaxies.push(coord);
                    col_counter[col] += 1;
                }
            }
        }

        let empty_cols = col_counter
            .iter()
            .enumerate()
            .filter(|(_, count)| **count == 0)
            .map(|(index, _)| index)
            .collect();

        // -- Build paths.
        let mut paths = Vec::new();
        let mut points = galaxies.clone();

        while let Some(begin) = points.pop() {
            for end in points.iter() {
                let begin = begin.clone();
                let end = end.clone();
                let edge = Edge::from((begin, end));
                paths.push(edge);
            }
        }

        // -- Return structure.
        Self {
            empty_cols,
            empty_rows,
            paths,
        }
    }
}

impl Image {
    fn manhattan_distance(&self, edge: &Edge, adjust_amount: usize) -> usize {
        let begin = &edge.begin;
        let end = &edge.end;

        let col_range = min(begin.col, end.col)..=max(begin.col, end.col);
        let row_range = min(begin.row, end.row)..=max(begin.row, end.row);

        let col_adjust = self
            .empty_cols
            .iter()
            .filter(|col| col_range.contains(col))
            .count();
        let row_adjust = self
            .empty_rows
            .iter()
            .filter(|row| row_range.contains(row))
            .count();

        let begin_col = begin.col as i64;
        let begin_row = begin.row as i64;
        let end_col = end.col as i64;
        let end_row = end.row as i64;

        let distance = (end_col - begin_col).abs() + (end_row - begin_row).abs();
        let distance: usize = distance.try_into().unwrap();
        let adjust_amount = adjust_amount - 1;
        distance + (col_adjust * adjust_amount) + (row_adjust * adjust_amount)
    }

    fn sum_of_distances(&self, adjust_amount: usize) -> usize {
        self.paths
            .iter()
            .map(|edge| self.manhattan_distance(edge, adjust_amount))
            .sum()
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Image {
    Image::from(input)
}

#[aoc(day11, part1)]
fn part1(image: &Image) -> usize {
    image.sum_of_distances(2)
}

#[aoc(day11, part2)]
fn part2(image: &Image) -> usize {
    image.sum_of_distances(1_000_000)
}
