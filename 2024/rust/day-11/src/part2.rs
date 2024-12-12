use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Stone(u64);

const N_BLINKS: usize = 75;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut stones = input
        .split_whitespace()
        .map(|dig| Stone(dig.parse().unwrap()))
        .collect::<Vec<_>>();

    let mut cache = HashMap::<_, _>::new();
    Ok(stones
        .iter_mut()
        .map(|s| s.stones_after_blinking_n_times(N_BLINKS, &mut cache))
        .sum::<usize>()
        .to_string())
}

impl Stone {
    fn apply_rule(&self) -> (Self, Option<Self>) {
        match self.0 {
            0 => (Stone(1), None),
            _ if self.has_even_digits() => {
                let (first, second) = self.split_into_two().unwrap();
                (first, Some(second))
            }
            _ => (Stone(self.0 * 2024), None),
        }
    }

    fn split_into_two(&self) -> Option<(Self, Self)> {
        let d = self.count_digits();
        if d % 2 != 0 || self.0 == 0 {
            return None;
        }
        let first = self.0 / 10u64.pow(d / 2);
        let second = self.0 - (first * 10u64.pow(d / 2));
        Some((Stone(first), Stone(second)))
    }

    fn count_digits(&self) -> u32 {
        self.0.checked_ilog10().unwrap_or_default() + 1
    }

    fn has_even_digits(&self) -> bool {
        self.0 != 0 && self.count_digits() % 2 == 0
    }

    fn stones_after_blinking_n_times(
        &self,
        n: usize,
        cache: &mut HashMap<(Stone, usize), usize>,
    ) -> usize {
        if n == 0 {
            return 1;
        }
        let mut result = 0;
        if !cache.contains_key(&(self.clone(), n)) {
            let (first, maybe_second) = self.apply_rule();
            result += first.stones_after_blinking_n_times(n - 1, cache);
            if let Some(second) = maybe_second {
                result += second.stones_after_blinking_n_times(n - 1, cache);
            }
            cache.insert((self.clone(), n), result);
        }
        *cache.get(&(self.clone(), n)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", process(input)?);
        Ok(())
    }

    #[test]
    fn test_has_even_digits() -> miette::Result<()> {
        assert!(!Stone(0).has_even_digits());
        assert!(!Stone(1).has_even_digits());
        assert!(!Stone(5).has_even_digits());
        assert!(!Stone(9).has_even_digits());
        assert!(Stone(10).has_even_digits());
        assert!(Stone(25).has_even_digits());
        assert!(!Stone(100).has_even_digits());
        assert!(!Stone(567).has_even_digits());
        assert!(Stone(1000).has_even_digits());
        assert!(Stone(6785).has_even_digits());
        assert!(Stone(3456).has_even_digits());
        assert!(Stone(9834).has_even_digits());

        Ok(())
    }

    #[test]
    fn test_count_digits() -> miette::Result<()> {
        assert_eq!(Stone(0).count_digits(), 1);
        assert_eq!(Stone(1).count_digits(), 1);
        assert_eq!(Stone(10).count_digits(), 2);
        assert_eq!(Stone(100).count_digits(), 3);

        Ok(())
    }

    #[test]
    fn test_split_into_two() -> miette::Result<()> {
        assert_eq!(Stone(22).split_into_two(), Some((Stone(2), Stone(2))));
        assert_eq!(Stone(15).split_into_two(), Some((Stone(1), Stone(5))));
        assert_eq!(Stone(1234).split_into_two(), Some((Stone(12), Stone(34))));
        assert_eq!(Stone(9876).split_into_two(), Some((Stone(98), Stone(76))));
        assert_eq!(Stone(222).split_into_two(), None);
        assert_eq!(Stone(1).split_into_two(), None);
        assert_eq!(Stone(0).split_into_two(), None);

        Ok(())
    }
}
