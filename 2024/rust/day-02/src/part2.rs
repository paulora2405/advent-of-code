use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

type Report = Vec<i32>;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse_input(input).map_err(|e| miette!("parse failed {}", e))?;

    let mut safe_reports = 0;

    for report in reports {
        if is_report_safe(&report, true) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}

#[tracing::instrument(ret)]
fn is_report_safe(report: &[i32], damper_on: bool) -> bool {
    let differences = report.windows(2).map(|a| a[0] - a[1]).collect::<Vec<_>>();
    let is_increasing = differences.first().unwrap() > &0;
    for diff in differences.iter() {
        if is_increasing && !(1..=3).contains(diff) || !is_increasing && !(-3..=-1).contains(diff) {
            if damper_on {
                for to_drop in 0..report.len() {
                    let mut damped_report = report.to_vec();
                    damped_report.remove(to_drop);
                    if is_report_safe(&damped_report, false) {
                        return true;
                    }
                }
                return false;
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_edge_case() -> miette::Result<()> {
        let input = "3 2 3 4 5
1 2 3 4 3";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
