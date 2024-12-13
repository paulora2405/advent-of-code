use glam::I64Vec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

const OFFSET: I64Vec2 = I64Vec2::new(10_000_000_000_000, 10_000_000_000_000);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MachineSpec {
    dx_a: I64Vec2,
    dx_b: I64Vec2,
    prize: I64Vec2,
}

impl MachineSpec {
    fn fewest_tokens_to_prize(&self) -> Option<i64> {
        let (a_x, a_y) = (self.dx_a.x, self.dx_a.y);
        let (b_x, b_y) = (self.dx_b.x, self.dx_b.y);
        let (p_x, p_y) = (self.prize.x, self.prize.y);

        let a_presses = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x);
        let b_presses = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);
        if a_presses * a_x + b_presses * b_x == p_x && a_presses * a_y + b_presses * b_y == p_y {
            Some(a_presses * 3 + b_presses)
        } else {
            None
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machine_specs) =
        parse_input(input).map_err(|e| miette!("failed to parse input {e}"))?;

    Ok(machine_specs
        .iter()
        .filter_map(|s| s.fewest_tokens_to_prize())
        .sum::<i64>()
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
                        preceded(tag("X+"), complete::i64),
                        tag(", "),
                        preceded(tag("Y+"), complete::i64),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| I64Vec2::new(x, y)),
            terminated(
                preceded(
                    tag("Button B: "),
                    separated_pair(
                        preceded(tag("X+"), complete::i64),
                        tag(", "),
                        preceded(tag("Y+"), complete::i64),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| I64Vec2::new(x, y)),
            terminated(
                preceded(
                    tag("Prize: "),
                    separated_pair(
                        preceded(tag("X="), complete::i64),
                        tag(", "),
                        preceded(tag("Y="), complete::i64),
                    ),
                ),
                line_ending,
            )
            .map(|(x, y)| I64Vec2::new(x, y)),
        )),
        |(v1, v2, v3)| MachineSpec {
            dx_a: v1,
            dx_b: v2,
            prize: v3 + OFFSET,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // Machine 1 = 280 tokens (A*80 B*40)
        // Machine 2 = None
        // Machine 3 = 200 tokens (A*38 B*86)
        // Machine 4 = None
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
        assert_eq!("875318608908", process(input)?);
        Ok(())
    }
}
