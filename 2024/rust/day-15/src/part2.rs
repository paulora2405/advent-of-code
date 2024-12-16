use glam::IVec2;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Default)]
struct Grid {
    robot: IVec2,
    boxes: HashSet<IVec2>,
    walls: HashSet<IVec2>,
    size: IVec2,
}

#[derive(Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut grid, moves) = parse_input(input);

    for m in &moves {
        grid.try_moving_to(&(grid.robot + m.dx()), m, false);
    }

    Ok(grid.get_gps_sum().to_string())
}

impl Grid {
    fn get_gps_sum(&self) -> i32 {
        self.boxes.iter().map(|b| b.x + b.y * 100).sum()
    }

    fn try_moving_to(&mut self, new_pos: &IVec2, movement: &Move, is_moving_box: bool) -> bool {
        // TODO: actually solve part 2
        // no need to bound check because borders always have walls
        if self.walls.contains(new_pos) {
            return false;
        }
        if self.boxes.contains(new_pos) {
            let should_move = self.try_moving_to(&(new_pos + movement.dx()), movement, true);
            if should_move {
                if is_moving_box {
                    self.boxes.remove(&(new_pos - movement.dx()));
                    self.boxes.insert(*new_pos);
                } else {
                    self.robot = *new_pos;
                }
            }
            should_move
        } else {
            if is_moving_box {
                self.boxes.remove(&(new_pos - movement.dx()));
                self.boxes.insert(*new_pos);
            } else {
                self.robot = *new_pos;
            }
            true
        }
    }
}

impl Move {
    fn dx(&self) -> IVec2 {
        match self {
            Move::Up => IVec2::new(0, -1),
            Move::Right => IVec2::new(1, 0),
            Move::Down => IVec2::new(0, 1),
            Move::Left => IVec2::new(-1, 0),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::with_capacity(((self.size.x + 1) * self.size.y) as usize);

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let c;
                let pos = IVec2::new(x, y);
                if self.robot == pos {
                    c = '@';
                } else if self.boxes.contains(&pos) {
                    c = 'O';
                } else if self.walls.contains(&pos) {
                    c = '#';
                } else {
                    c = '.';
                }
                text += &c.to_string();
            }
            text += "\n"
        }

        write!(f, "{text}")
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Up => write!(f, "^"),
            Move::Right => write!(f, ">"),
            Move::Down => write!(f, "v"),
            Move::Left => write!(f, "<"),
        }
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Move>) {
    let (grid_str, moves) = input.split_once("\n\n").unwrap();
    let moves = moves
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Move::Up,
                '>' => Move::Right,
                'v' => Move::Down,
                '<' => Move::Left,
                _ => unreachable!(),
            })
        })
        .collect_vec();

    let mut grid = Grid {
        size: IVec2::new(
            grid_str.lines().count() as i32,
            grid_str.lines().next().unwrap().chars().count() as i32,
        ),
        ..Default::default()
    };

    grid_str.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '@' => grid.robot = IVec2::new(x as i32, y as i32),
            'O' => {
                let _ = grid.boxes.insert(IVec2::new(x as i32, y as i32));
            }
            '#' => {
                let _ = grid.walls.insert(IVec2::new(x as i32, y as i32));
            }
            '.' => (),
            _ => unreachable!(),
        });
    });

    (grid, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

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
