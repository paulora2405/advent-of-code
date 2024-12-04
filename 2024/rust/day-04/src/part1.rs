#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut words_found = 0;
    let offsets = [-1, 0, 1];
    let letters = ['X', 'M', 'A', 'S'];

    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            for row_off in offsets {
                for col_off in offsets {
                    if letters_match(&lines, &letters, (row, col), (row_off, col_off)) {
                        words_found += 1;
                    }
                }
            }
        }
    }

    Ok(words_found.to_string())
}

fn letters_match(
    lines: &Vec<Vec<char>>,
    letters: &[char],
    (row, col): (usize, usize),
    (offset_row, offset_col): (isize, isize),
) -> bool {
    if lines.get(row).is_some()
        && lines[row].get(col).is_some()
        && letters.first().is_some_and(|&c| c == lines[row][col])
    {
        if letters.len() == 1 {
            return true;
        }
        if let (Some(new_row), Some(new_col)) = (
            row.checked_add_signed(offset_row),
            col.checked_add_signed(offset_col),
        ) {
            return letters_match(
                lines,
                &letters[1..],
                (new_row, new_col),
                (offset_row, offset_col),
            );
        }
    }
    false
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
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test]
    fn test_simple() -> miette::Result<()> {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
