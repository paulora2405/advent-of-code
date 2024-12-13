use glam::UVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

const MAX_PRESSES: u32 = 200;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MachineSpec {
    dx_a: UVec2,
    dx_b: UVec2,
    prize_pos: UVec2,
}

impl MachineSpec {
    fn fewest_tokens_to_prize(&self) -> Option<u32> {
        for presses in 1..=MAX_PRESSES {
            for i in 0..=presses {
                let a_presses = i;
                let b_presses = presses - i;
                let final_x = self.dx_a.x * a_presses + self.dx_b.x * b_presses;
                let final_y = self.dx_a.y * a_presses + self.dx_b.y * b_presses;
                if final_x == self.prize_pos.x && final_y == self.prize_pos.y {
                    return Some(a_presses * 3 + b_presses);
                }
            }
        }
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machine_specs) =
        parse_input(input).map_err(|e| miette!("failed to parse input {e}"))?;

    Ok(machine_specs
        .iter()
        .filter_map(|s| s.fewest_tokens_to_prize())
        .sum::<u32>()
        .to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<MachineSpec>> {
    separated_list1(line_ending, parse_machine_spec)(input)
}

fn parse_machine_spec(input: &str) -> IResult<&str, MachineSpec> {
    map(
        tuple((
            terminated(
                preceded(
                    tag("Button A: "),
                    separated_pair(
                        preceded(tag("X+"), complete::u32),
                        tag(", "),
                        preceded(tag("Y+"), complete::u32),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| UVec2::new(x, y)),
            terminated(
                preceded(
                    tag("Button B: "),
                    separated_pair(
                        preceded(tag("X+"), complete::u32),
                        tag(", "),
                        preceded(tag("Y+"), complete::u32),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| UVec2::new(x, y)),
            terminated(
                preceded(
                    tag("Prize: "),
                    separated_pair(
                        preceded(tag("X="), complete::u32),
                        tag(", "),
                        preceded(tag("Y="), complete::u32),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| UVec2::new(x, y)),
        )),
        |(v1, v2, v3)| MachineSpec {
            dx_a: v1,
            dx_b: v2,
            prize_pos: v3,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
