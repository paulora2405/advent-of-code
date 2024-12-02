use std::iter::zip;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut col1 = vec![];
    let mut col2 = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        col1.push(parts.next().unwrap().parse::<i32>().unwrap());
        col2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }
    col1.sort();
    col2.sort();

    let result = zip(col1, col2).map(|(a, b)| (a - b).abs()).sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
