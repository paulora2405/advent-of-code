use std::{cmp::max, collections::HashSet};

use miette::miette;

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
    let mut curr_pos = (0, 0);
    let mut dir = Dir::Up;
    let mut visited = HashSet::<((usize, usize), Dir)>::new();

    lines.enumerate().for_each(|(i, line)| {
        n = max(n, i + 1);
        line.chars().enumerate().for_each(|(j, c)| {
            m = max(m, j + 1);
            if c == '#' {
                obstacles.insert((i, j));
            } else if c == '^' {
                curr_pos = (i, j);
            }
        });
    });

    loop {
        visited.insert((curr_pos, dir));
        let next_pos = dir.next(curr_pos, n, m);

        if next_pos.is_some() {
            if obstacles.contains(&next_pos.unwrap()) {
                dir.rotate();
            } else if visited.contains(&(next_pos.unwrap(), dir)) {
                break;
            } else {
                curr_pos = next_pos.unwrap();
            }
        } else {
            break;
        }
    }

    Ok(visited
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
        .to_string())
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
