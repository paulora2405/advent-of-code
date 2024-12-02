use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut col1 = vec![];
    let mut col2 = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        col1.push(parts.next().unwrap().parse::<i32>().unwrap());
        col2.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    let mut counts = HashMap::new();
    for &num in &col2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    let result = col1
        .iter()
        .map(|a| a * counts.get(a).unwrap_or(&0))
        .sum::<i32>();

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
