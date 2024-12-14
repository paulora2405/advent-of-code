use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

const N: i32 = 101;
const M: i32 = 103;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, robots) = parse_input(input).map_err(|e| miette!("failed to parse input {e}"))?;

    for i in 0..N * M {
        let positions = robots
            .iter()
            .map(|r| r.position_after_n_seconds(i))
            .collect_vec();

        let overlaps = Robot::total_overlapping_robots(&positions);
        if overlaps == 0 {
            println!("{}", Robot::_robots_to_string(&positions));
            return Ok(i.to_string());
        }
    }

    panic!("could not find moment with easter egg")
}

impl Robot {
    fn position_after_n_seconds(&self, n: i32) -> IVec2 {
        let x = (self.pos.x + (self.vel.x * n)).rem_euclid(N);
        let y = (self.pos.y + (self.vel.y * n)).rem_euclid(M);
        IVec2 { x, y }
    }

    fn total_overlapping_robots(robots_pos: &[IVec2]) -> usize {
        let mut seen_positions = HashSet::<IVec2>::new();
        robots_pos
            .iter()
            .filter(|&pos| !seen_positions.insert(*pos))
            .count()
    }

    fn _robots_to_string(positions: &[IVec2]) -> String {
        let mut text = String::with_capacity(((N + 1) * M) as usize);
        for y in 0..M {
            for x in 0..N {
                let count = positions.iter().filter(|p| p.x == x && p.y == y).count();
                if count > 0 {
                    text += &count.to_string();
                } else {
                    text += ".";
                }
            }
            text += "\n"
        }
        text
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_robot)(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        separated_pair(
            preceded(
                tag("p="),
                separated_pair(complete::i32, tag(","), complete::i32),
            )
            .map(|(x, y)| IVec2 { x, y }),
            space1,
            preceded(
                tag("v="),
                separated_pair(complete::i32, tag(","), complete::i32),
            )
            .map(|(x, y)| IVec2 { x, y }),
        ),
        |(pos, vel)| Robot { pos, vel },
    )(input)
}

#[cfg(test)]
mod tests {
    //! This challange does not have an anwser to the example.
}
