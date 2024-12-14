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

#[derive(Debug, Hash, PartialEq, Eq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottonRight,
}

#[cfg(test)]
const N: i32 = 11;
#[cfg(test)]
const M: i32 = 7;
#[cfg(not(test))]
const N: i32 = 101;
#[cfg(not(test))]
const M: i32 = 103;

const VERTICAL_LINE: i32 = N / 2;
const HORIZONTAL_LINE: i32 = M / 2;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, robots) = parse_input(input).map_err(|e| miette!("failed to parse input {e}"))?;

    let in_each_q = robots
        .iter()
        .filter_map(|r| {
            let final_pos = r.position_after_n_seconds(100);
            Quadrant::get(&final_pos)
        })
        .counts();

    // println!(
    //     "{}",
    //     Robot::_robots_to_string(
    //         &robots
    //             .iter()
    //             .map(|r| r.position_after_n_seconds(100))
    //             .collect_vec()
    //     )
    // );

    // dbg!(&in_each_q);

    Ok(in_each_q
        .into_values()
        .reduce(|acc, v| acc * v)
        .unwrap()
        .to_string())
}

impl Quadrant {
    fn get(pos: &IVec2) -> Option<Self> {
        match (pos.x, pos.y) {
            (x, y) if x < VERTICAL_LINE && y < HORIZONTAL_LINE => Some(Self::TopLeft),
            (x, y) if x > VERTICAL_LINE && y < HORIZONTAL_LINE => Some(Self::TopRight),
            (x, y) if x < VERTICAL_LINE && y > HORIZONTAL_LINE => Some(Self::BottomLeft),
            (x, y) if x > VERTICAL_LINE && y > HORIZONTAL_LINE => Some(Self::BottonRight),
            _ => None,
        }
    }
}

impl Robot {
    fn position_after_n_seconds(&self, n: i32) -> IVec2 {
        let x = (self.pos.x + (self.vel.x * n)).rem_euclid(N);
        let y = (self.pos.y + (self.vel.y * n)).rem_euclid(M);
        IVec2 { x, y }
    }

    fn _robots_to_string(positions: &[IVec2]) -> String {
        let mut text = String::with_capacity(((N + 1) * M) as usize);
        for y in 0..M {
            for x in 0..N {
                if y == HORIZONTAL_LINE || x == VERTICAL_LINE {
                    text += " ";
                } else {
                    let count = positions.iter().filter(|p| p.x == x && p.y == y).count();
                    if count > 0 {
                        text += &count.to_string();
                    } else {
                        text += ".";
                    }
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
    use super::*;

    #[test]
    fn test_simple() -> miette::Result<()> {
        let input = "p=2,4 v=2,-3";
        let (_, robot) = parse_robot(input).map_err(|e| miette!("failed to parse {e}"))?;

        assert_eq!(IVec2 { x: 1, y: 3 }, robot.position_after_n_seconds(5));

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("1a2", process(input)?);
        Ok(())
    }
}
