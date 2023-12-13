use aoc_runner_derive::{aoc, aoc_generator};

type Grid = Vec<Vec<bool>>;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut curr_rows = Vec::new();

    for line in input.lines() {
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

fn find_mirror(grid: &Grid) -> usize {
    for row_index in 1..grid.len() {
        let above = &grid[..row_index];
        let below = &grid[row_index..];

        if above
            .iter()
            .rev()
            .zip(below.iter())
            .all(|(above, below)| above == below)
        {
            return row_index;
        }
    }

    0
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

fn score(grid: &Grid, f: fn(&Grid) -> usize) -> usize {
    (f(grid) * 100) + f(&rotate_90(grid))
}

#[aoc(day13, part1)]
fn part1(grids: &[Grid]) -> usize {
    grids.iter().map(|grid| score(grid, find_mirror)).sum()
}

#[aoc(day13, part2)]
fn part2(grids: &[Grid]) -> usize {
    grids.iter().map(|grid| score(grid, find_mirror_smudge)).sum()
}
