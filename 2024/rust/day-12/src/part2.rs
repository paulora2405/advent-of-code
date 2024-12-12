use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use miette::miette;
use ndarray::Array2;

const ADJ4: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DOWN_RIGHT: [(i32, i32); 2] = [(1, 0), (0, 1)];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let n = input.lines().count();
    let m = input.lines().next().unwrap().chars().count();
    let grid = input
        .lines()
        .flat_map(|l| l.chars().collect_vec())
        .collect_vec();
    let grid = Array2::from_shape_vec((n, m), grid)
        .map_err(|e| miette!("failed to build 2D array {e}"))?;

    let mut regions = HashMap::<_, _>::with_capacity(n * m);

    let mut region_id = 0;
    for i in 0..n {
        for j in 0..m {
            if !regions.contains_key(&(i as i32, j as i32)) {
                find_region(
                    i as i32,
                    j as i32,
                    &grid[[i, j]],
                    region_id,
                    &grid,
                    &mut regions,
                );
                region_id += 1;
            }
        }
    }

    let inverted_regions = regions.iter().fold(
        HashMap::<i32, Vec<(i32, i32)>>::new(),
        |mut acc, (pos, id)| {
            acc.entry(*id)
                .and_modify(|e| e.push(*pos))
                .or_insert(vec![*pos]);
            acc
        },
    );

    let mut total_cost = 0;
    for (_id, nodes) in inverted_regions {
        let area = nodes.len();
        let mut fences = HashSet::<_>::new();
        for node in &nodes {
            for (dx, dy) in ADJ4 {
                let (nx, ny) = (node.0 + dx, node.1 + dy);
                if grid.get([nx as usize, ny as usize]).is_none() || !nodes.contains(&(nx, ny)) {
                    fences.insert((*node, (nx, ny)));
                }
            }
        }
        let mut straight_sides = HashSet::<_>::new();

        for (p1, p2) in &fences {
            let mut to_keep = true;
            for (dx, dy) in DOWN_RIGHT {
                let p1n = (p1.0 + dx, p1.1 + dy);
                let p2n = (p2.0 + dx, p2.1 + dy);
                if fences.contains(&(p1n, p2n)) {
                    to_keep = false;
                }
            }
            if to_keep {
                straight_sides.insert((p1, p2));
            }
        }

        total_cost += area * straight_sides.len();
    }

    Ok(total_cost.to_string())
}

fn find_region(
    x: i32,
    y: i32,
    initial_plot_type: &char,
    region_id: i32,
    grid: &Array2<char>,
    regions: &mut HashMap<(i32, i32), i32>,
) {
    if let Some(curr_plot_type) = grid.get([x as usize, y as usize]) {
        if !regions.contains_key(&(x, y)) && initial_plot_type == curr_plot_type {
            regions.insert((x, y), region_id);
            for (dx, dy) in ADJ4 {
                find_region(x + dx, y + dy, initial_plot_type, region_id, grid, regions);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_simple() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("80", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example_ab() -> miette::Result<()> {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("368", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example_ex() -> miette::Result<()> {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!("236", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example_medium() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("436", process(input)?);
        Ok(())
    }

    #[test]
    fn test_example_large() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
