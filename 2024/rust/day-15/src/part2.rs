use glam::IVec2;
use itertools::Itertools;
use ndarray::Array2;
use std::mem::swap;

const UP: IVec2 = IVec2::new(0, -1);
const DOWN: IVec2 = IVec2::new(0, 1);
const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut grid, moves) = parse_input(input);
    let mut position = grid
        .indexed_iter()
        .find(|(_, &c)| c == '@')
        .map(|((y, x), _)| IVec2::new(x as i32, y as i32))
        .unwrap();

    grid[position.to_index()] = '.';

    for m in &moves {
        match *m {
            UP | DOWN => {
                try_moving_vertically(&mut grid, &mut position, m);
            }
            LEFT | RIGHT => {
                try_moving_horizontally(&mut grid, &mut position, m);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", grid_to_string(&grid));

    Ok(get_gps_sum(&grid).to_string())
}

fn get_gps_sum(grid: &Array2<char>) -> usize {
    grid.indexed_iter()
        .filter_map(|((row, col), &c)| {
            if c == '[' {
                Some(row * 100 + col)
            } else {
                None
            }
        })
        .sum()
}

fn try_moving_horizontally(grid: &mut Array2<char>, start_pos: &mut IVec2, movement: &IVec2) {
    let mut position = *start_pos + movement;
    let mut size = 1;

    while grid[position.to_index()] != '.' && grid[position.to_index()] != '#' {
        position += movement;
        size += 1;
    }

    if grid[position.to_index()] == '.' {
        let mut previous = '.';
        let mut position = *start_pos + movement;

        for _ in 0..size {
            swap(&mut previous, &mut grid[position.to_index()]);
            position += movement;
        }

        *start_pos += movement;
    }
}

fn try_moving_vertically(grid: &mut Array2<char>, start_pos: &mut IVec2, movement: &IVec2) {
    if grid[(*start_pos + movement).to_index()] == '.' {
        *start_pos += movement;
        return;
    }

    let mut todo = Vec::with_capacity(50);
    todo.push(IVec2::ZERO);
    todo.push(*start_pos);
    let mut index = 1;

    while index < todo.len() {
        let next = todo[index] + movement;
        index += 1;

        let (first, second) = match grid[next.to_index()] {
            '[' => (next, next + RIGHT),
            ']' => (next + LEFT, next),
            '#' => return,
            _ => continue,
        };

        if first != todo[todo.len() - 2] {
            todo.push(first);
            todo.push(second);
        }
    }

    for &pos in todo[2..].iter().rev() {
        grid[(pos + movement).to_index()] = grid[pos.to_index()];
        grid[pos.to_index()] = '.';
    }

    *start_pos += movement;
}

trait ToIndex {
    fn to_index(&self) -> (usize, usize);
}

impl ToIndex for IVec2 {
    fn to_index(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}

fn grid_to_string(grid: &Array2<char>) -> String {
    grid.indexed_iter()
        .map(|(pos, &c)| {
            if pos.1 == 0 {
                "\n".to_string() + c.to_string().as_str()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn parse_input(input: &str) -> (Array2<char>, Vec<IVec2>) {
    let (grid_str, moves) = input.split_once("\n\n").unwrap();
    let moves = moves
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => UP,
                '>' => RIGHT,
                'v' => DOWN,
                '<' => LEFT,
                _ => unreachable!(),
            })
        })
        .collect_vec();

    let size = [
        grid_str.lines().count(),
        grid_str.lines().next().unwrap().chars().count(),
    ];

    let normal_grid = grid_str
        .lines()
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();
    let normal_grid = Array2::from_shape_vec(size, normal_grid).unwrap();

    println!("{}", grid_to_string(&normal_grid));

    let mut stretched_grid = Array2::from_elem([size[0], size[1] * 2], '.');

    for ((row, col), c) in normal_grid.indexed_iter() {
        let (left, right) = match c {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '@' => ('@', '.'),
            _ => continue,
        };
        stretched_grid[(row, col * 2)] = left;
        stretched_grid[(row, col * 2 + 1)] = right;
    }
    println!("{}", grid_to_string(&stretched_grid));

    (stretched_grid, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() -> miette::Result<()> {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";
        assert_eq!("618", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
