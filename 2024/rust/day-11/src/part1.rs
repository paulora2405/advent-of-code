#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Stone(u64);

impl Stone {
    fn apply_rule(&mut self) -> Option<Self> {
        match self.0 {
            0 => {
                self.0 = 1;
                None
            }
            _ if self.has_even_digits() => {
                let (first, second) = self.split_into_two().unwrap();
                *self = first;
                Some(second)
            }
            _ => {
                self.0 *= 2024;
                None
            }
        }
    }

    fn split_into_two(&self) -> Option<(Self, Self)> {
        let d = self.count_digits();
        if d % 2 != 0 || self.0 == 0 {
            return None;
        }
        let first = {
            let mut first = self.0;
            for _ in 0..d / 2 {
                first /= 10;
            }
            first
        };
        let second = { self.0 - (first * 10u64.pow(d / 2)) };

        Some((Stone(first), Stone(second)))
    }

    fn count_digits(&self) -> u32 {
        let mut digits = 0;
        let mut n = self.0;
        while n != 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }

    fn has_even_digits(&self) -> bool {
        self.0 != 0 && self.count_digits() % 2 == 0
    }

    fn blink_n_times(mut stones: Vec<Self>, n: usize) -> Vec<Self> {
        for _ in 0..n {
            let mut new_stones = vec![];
            for (i, s) in stones.iter_mut().enumerate() {
                if let Some(new_stone) = s.apply_rule() {
                    new_stones.push((i, new_stone));
                }
            }
            for (j, (i, new_stone)) in new_stones.into_iter().enumerate() {
                // technically, we can just push at the end, the order does not matter
                stones.insert(i + 1 + j, new_stone);
            }
        }
        stones
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut stones = input
        .split_whitespace()
        .map(|dig| Stone(dig.parse().unwrap()))
        .collect::<Vec<_>>();

    stones = Stone::blink_n_times(stones, 25);

    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_examples() -> miette::Result<()> {
        let initial_arr = vec![125, 17].into_iter().map(Stone).collect_vec();
        let after_1_arr = vec![253000, 1, 7].into_iter().map(Stone).collect_vec();
        let after_2_arr = vec![253, 0, 2024, 14168].into_iter().map(Stone).collect_vec();
        let after_3_arr = vec![512072, 1, 20, 24, 28676032].into_iter().map(Stone).collect_vec();
        let after_4_arr = vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032].into_iter().map(Stone).collect_vec();
        let after_5_arr = vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32].into_iter().map(Stone).collect_vec();
        let after_6_arr = vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2].into_iter().map(Stone).collect_vec();

        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 1), after_1_arr);
        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 2), after_2_arr);
        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 3), after_3_arr);
        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 4), after_4_arr);
        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 5), after_5_arr);
        assert_eq!(Stone::blink_n_times(initial_arr.clone(), 6), after_6_arr);

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
