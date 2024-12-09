//! This solution was taken from the amazing collection of solutions from
//! https://github.com/maneatingape
use nom::ToUsize;
use std::array::from_fn;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disk = parse_input(input);

    let mut block = 0;
    let mut checksum = 0;
    let mut free: [_; 10] = from_fn(|_| BinaryHeap::with_capacity(1_000));

    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(Reverse(block));
        }

        block += size;
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        block -= size;

        if index % 2 == 1 {
            continue;
        }

        let mut next_block = block;
        let mut next_index = usize::MAX;

        #[allow(clippy::needless_range_loop)]
        for i in size..10 {
            if let Some(&Reverse(first)) = free[i].peek() {
                if first < next_block {
                    next_block = first;
                    next_index = i;
                }
            }
        }

        let id = index / 2;
        let extra = next_block * size + EXTRA[size];
        checksum += id * extra;

        if next_index != usize::MAX {
            free[next_index].pop();
            if size < next_index {
                free[next_index - size].push(Reverse(next_block + size));
            }
        }
    }

    Ok(checksum.to_string())
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n.to_usize())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
