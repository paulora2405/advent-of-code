use rayon::prelude::*;
use std::{cmp::max, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn next(&self, pos: (usize, usize), n: usize, m: usize) -> Option<(usize, usize)> {
        match self {
            Dir::Up if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
            Dir::Right if pos.1 < m - 1 => Some((pos.0, pos.1 + 1)),
            Dir::Down if pos.0 < n - 1 => Some((pos.0 + 1, pos.1)),
            Dir::Left if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
            _ => None,
        }
    }

    fn rotate(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input.lines();

    let (mut n, mut m) = (0, 0);

    let mut obstacles = HashSet::<(usize, usize)>::new();
    let mut init_pos = (0, 0);
    let init_dir = Dir::Up;

    lines.enumerate().for_each(|(i, line)| {
        n = max(n, i + 1);
        line.chars().enumerate().for_each(|(j, c)| {
            m = max(m, j + 1);
            if c == '#' {
                obstacles.insert((i, j));
            } else if c == '^' {
                init_pos = (i, j);
            }
        });
    });

    let combinations: Vec<(usize, usize)> =
        (0..n).flat_map(|i| (0..m).map(move |j| (i, j))).collect();

    let positions_that_generate_loops: i32 = combinations
        .par_iter()
        .fold(
            || 0i32,
            |acc, (i, j)| {
                let mut obstacles = obstacles.clone();
                if init_pos != (*i, *j) && !obstacles.contains(&(*i, *j)) {
                    obstacles.insert((*i, *j));
                    if move_guard(init_pos, init_dir, &obstacles, n, m) {
                        acc + 1
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            },
        )
        .sum();

    Ok(positions_that_generate_loops.to_string())
}

fn move_guard(
    init_pos: (usize, usize),
    init_dir: Dir,
    obstacles: &HashSet<(usize, usize)>,
    n: usize,
    m: usize,
) -> bool {
    let mut curr_pos = init_pos;
    let mut dir = init_dir;
    let mut visited = HashSet::<((usize, usize), Dir)>::new();
    loop {
        visited.insert((curr_pos, dir));
        let next_pos = dir.next(curr_pos, n, m);

        if next_pos.is_some() {
            if obstacles.contains(&next_pos.unwrap()) {
                dir.rotate();
            } else if visited.contains(&(next_pos.unwrap(), dir)) {
                return true;
            } else {
                curr_pos = next_pos.unwrap();
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
