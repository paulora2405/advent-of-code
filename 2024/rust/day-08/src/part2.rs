use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let n = input.lines().count() as i32;
    let m = input.lines().next().unwrap().len() as i32;
    let antennas = parse_input(input);
    let mut antinodes = HashSet::<_>::new();

    for positions in antennas.values() {
        positions
            .iter()
            .cloned()
            .tuple_combinations()
            .for_each(|(a, b)| {
                let diff = a - b;
                antinodes.insert(a);
                antinodes.insert(b);

                let mut after = a + diff;
                let mut before = b - diff;
                while (0..n).contains(&after.y) && (0..m).contains(&after.x) {
                    antinodes.insert(after);
                    after += diff;
                }
                while (0..n).contains(&before.y) && (0..m).contains(&before.x) {
                    antinodes.insert(before);
                    before -= diff;
                }
            });
    }

    Ok(antinodes.len().to_string())
}

fn parse_input(input: &str) -> HashMap<char, HashSet<IVec2>> {
    let mut antennas = HashMap::<_, HashSet<_>>::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|locs| {
                        locs.insert(IVec2::new(i as i32, j as i32));
                    })
                    .or_insert(HashSet::<_>::from([IVec2::new(i as i32, j as i32)]));
            }
        })
    });
    antennas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
