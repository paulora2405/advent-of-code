#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut words_found = 0;
    let letters = ["MS", "SM"];

    for row in 1..lines.len() - 1 {
        for col in 1..lines[0].len() - 1 {
            if lines[row][col] == 'A' {
                let diag1 = format!("{}{}", lines[row - 1][col - 1], lines[row + 1][col + 1]);
                let diag2 = format!("{}{}", lines[row - 1][col + 1], lines[row + 1][col - 1]);
                if (diag1 == letters[0] || diag1 == letters[1])
                    && (diag2 == letters[0] || diag2 == letters[1])
                {
                    words_found += 1;
                }
            }
        }
    }

    Ok(words_found.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
