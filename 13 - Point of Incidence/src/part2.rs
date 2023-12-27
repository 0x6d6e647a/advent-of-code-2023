use std::io::{stdin, BufRead, Stdin};

type Grid = Vec<Vec<bool>>;

fn rotate_90(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();

    for col_index in 0..grid[0].len() {
        let mut new_row = Vec::new();

        for row in grid {
            new_row.push(row[col_index]);
        }

        new_grid.push(new_row);
    }

    new_grid
}

fn find_mirror_smudge(grid: &Grid) -> usize {
    for row_index in 1..grid.len() {
        let above = &grid[..row_index];
        let below = &grid[row_index..];

        let mut num_smudges = 0;

        'row: for (above_row, below_row) in above.iter().rev().zip(below.iter()) {
            for (above_col, below_col) in above_row.iter().zip(below_row.iter()) {
                if above_col != below_col {
                    num_smudges += 1;
                }

                if num_smudges > 1 {
                    break 'row;
                }
            }
        }

        if num_smudges == 1 {
            return row_index;
        }
    }

    0
}

fn score(grid: &Grid) -> usize {
    (find_mirror_smudge(grid) * 100) + find_mirror_smudge(&rotate_90(grid))
}

fn parse_grids(stdin: &Stdin) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut curr_rows = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            grids.push(curr_rows);
            curr_rows = Vec::new();
        } else {
            curr_rows.push(
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("invalid character: '{}'", c),
                    })
                    .collect(),
            );
        }
    }

    if !curr_rows.is_empty() {
        grids.push(curr_rows);
    }

    grids
}

fn main() {
    let grids = parse_grids(&stdin());
    let sum: usize = grids.iter().map(|grid| score(grid)).sum();
    println!("{sum}");
}
