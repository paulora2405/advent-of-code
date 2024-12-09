//! # Disk Fragmenter
//!
//! ## Part One
//!
//! Computes the checksum by simultaneously scanning forward for free blocks and
//! backwards for files. No memory is allocated which makes it very fast.
//!
//! ## Part Two
//!
//! We build 10 [min heaps](https://en.wikipedia.org/wiki/Heap_(data_structure)) in an array to
//! store the free space offsets. The index of the array implicitly stores the size of the
//! free block.
//!
//! When moving a file to a free block, the corresponding heap is popped and then any leftover
//! space is pushed back to the heap at a smaller index. The heap at index zero is not used
//! but makes the indexing easier.

/// [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) offset by two.
/// Files can be a max size of 9 so we only need the first 10 values, including zero to make
/// indexing easier.
const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

/// Remove any trailing newlines and convert to `usize`.
pub fn parse(input: &str) -> Vec<usize> {
    input.trim().bytes().map(|b| (b - b'0') as usize).collect()
}

/// Block by block checksum comparison that doesn't allocate any memory.
pub fn part1(disk: &[usize]) -> usize {
    // Start at the first free block and the last file.
    let mut free = 0;
    let mut file = disk.len() + disk.len() % 2;

    let mut available = 0;
    let mut needed = 0;

    let mut block = 0;
    let mut checksum = 0;

    while free < file {
        // Take as much space as possible from the current free block range.
        let size = needed.min(available);
        (checksum, block) = update(checksum, block, file, size);
        available -= size;
        needed -= size;

        // One or both of "available" and "free" could be zero.
        if needed == 0 {
            file -= 2;
            needed = disk[file];
        }

        // When moving to the next free block, add the checksum for the file we're skipping over.
        if available == 0 {
            let size = disk[free];
            (checksum, block) = update(checksum, block, free, size);
            available = disk[free + 1];
            free += 2;
        }
    }

    // Account for any remaining file blocks left over.
    (checksum, _) = update(checksum, block, file, needed);
    checksum
}

/// Convenience function to update checksum based on file location and size.
#[inline]
fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block * size + EXTRA[size];
    (checksum + id * extra, block + size)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(part1(&parse(input)).to_string())
}

use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
