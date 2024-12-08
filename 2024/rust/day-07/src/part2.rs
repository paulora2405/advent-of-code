use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use tracing::info;

#[derive(Debug, Clone)]
enum Ops {
    Add,
    Mul,
    Cat,
}

impl Ops {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Ops::Add => a + b,
            Ops::Mul => a * b,
            Ops::Cat => concat_nums(a, b),
        }
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, equations) = parse_input(input).map_err(|e| miette!("failed to parse {}", e))?;

    Ok(equations
        .iter()
        .map(|(r, v)| {
            if is_solvable(*r, v, &[]) {
                info!("{v:?} is solvable to {r}");
                *r
            } else {
                info!("{v:?} is NOT solvable to {r}");
                0
            }
        })
        .sum::<u64>()
        .to_string())
}

// #[tracing::instrument(ret)]
fn is_solvable(expected_result: u64, values: &[u64], operations: &[Ops]) -> bool {
    if values.len() - 1 == operations.len() {
        let mut total = values[0];
        for i in 1..values.len() {
            total = operations[i - 1].apply(total, values[i]);
        }
        return total == expected_result;
    }

    let mut ops_add = operations.to_vec();
    ops_add.push(Ops::Add);
    let mut ops_mul = operations.to_vec();
    ops_mul.push(Ops::Mul);
    let mut ops_cat = operations.to_vec();
    ops_cat.push(Ops::Cat);

    if is_solvable(expected_result, values, &ops_add)
        || is_solvable(expected_result, values, &ops_mul)
        || is_solvable(expected_result, values, &ops_cat)
    {
        return true;
    }
    false
}

fn concat_nums(a: u64, b: u64) -> u64 {
    let a_str = a.to_string();
    let b_str = b.to_string();

    let concatenated = format!("{}{}", a_str, b_str);

    concatenated
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse concatenated number {}", concatenated))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
