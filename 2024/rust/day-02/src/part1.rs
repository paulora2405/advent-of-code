#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_reports = 0;

    for report in reports {
        safe_reports += if is_report_safe(&report) { 1 } else { 0 };
    }

    Ok(safe_reports.to_string())
}

fn is_report_safe(report: &[i32]) -> bool {
    let differences = report.windows(2).map(|a| a[0] - a[1]).collect::<Vec<_>>();
    let is_increasing = differences.first().unwrap() > &0;
    for diff in differences {
        if is_increasing && !(1..=3).contains(&diff) || !is_increasing && !(-3..=-1).contains(&diff)
        {
            return false;
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
