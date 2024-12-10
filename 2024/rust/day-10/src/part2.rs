use itertools::Itertools;

#[derive(Debug)]
struct Trail {
    n: i32,
    m: i32,
    grid: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> Vec<Self> {
        vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left]
    }

    fn going_from(&self, pos: (i32, i32)) -> (i32, i32) {
        let (i, j) = pos;
        match self {
            Dir::Up => (i - 1, j),
            Dir::Right => (i, j + 1),
            Dir::Down => (i + 1, j),
            Dir::Left => (i, j - 1),
        }
    }
}

impl Trail {
    fn get_altitude(&self, pos: (i32, i32)) -> Option<u32> {
        if (0..self.n).contains(&pos.0) && (0..self.m).contains(&pos.1) {
            Some(self.grid[pos.0 as usize][pos.1 as usize])
        } else {
            None
        }
    }

    fn is_hikable(&self, curr_pos: (i32, i32), dir: &Dir) -> bool {
        if let Some(curr_altitude) = self.get_altitude(curr_pos) {
            if let Some(next_altitude) = self.get_altitude(dir.going_from(curr_pos)) {
                if curr_altitude + 1 == next_altitude {
                    return true;
                }
            }
        }
        false
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let trail = Trail {
        n: input.lines().count() as i32,
        m: input.lines().next().unwrap().chars().count() as i32,
        grid: input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec(),
    };

    Ok(find_all_hike_trails(&trail).to_string())
}

fn find_all_hike_trails(trail: &Trail) -> u32 {
    let mut starts = vec![];
    for (i, line) in trail.grid.iter().enumerate() {
        for (j, altitude) in line.iter().enumerate() {
            if *altitude == 0 {
                starts.push((i as i32, j as i32));
            }
        }
    }

    starts.iter().map(|start| hike(trail, *start)).sum()
}

#[tracing::instrument]
fn hike(trail: &Trail, pos: (i32, i32)) -> u32 {
    if let Some(altitude) = trail.get_altitude(pos) {
        if altitude == 9 {
            return 1;
        }
    } else {
        return 0;
    }

    Dir::all()
        .iter()
        .map(|d| {
            if trail.is_hikable(pos, d) {
                hike(trail, d.going_from(pos))
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() -> miette::Result<()> {
        let input = "0123
1234
8765
9876";
        assert_eq!("16", process(input)?);
        Ok(())
    }

    #[test]
    fn test_large() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
