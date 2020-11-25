use std::cmp;
use core::fmt;

#[derive(Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {x: x, y: y}
    }

    fn manhattan_distance(self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    fn up() -> Position {
        Position{x:0, y:1}
    }

    fn down() -> Position {
        Position{x:0, y:-1}
    }

    fn right() -> Position {
        Position{x:1, y:0}
    }

    fn left() -> Position {
        Position{x:-1, y:0}
    }

    fn min(&self, other: &Self) -> Position {
        Position {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }

    fn max(&self, other: &Self) -> Position {
        Position {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Grid {
    grid: Vec<Position>,
    size: Position,
}

impl Grid {
    fn empty() -> Grid {
        Grid{
            grid: vec!(Position::new(0, 0)),
            size: Position{x:1, y:1},
        }
    }

    fn get(&self, i: u32) -> Option<Position> {
        let d = self.grid.get((i - 1) as usize);
        if d.is_none() {
            None
        } else {
            Some((*d.unwrap()).clone())
        }
    }

    fn new(until: u32) -> Grid {
        let mut grid = Grid::empty();
        let mut i: u32 = 2;
        let mut current= grid.grid[0].clone();
        let mut direction = Position::right();

        let mut min_pos = Position{x:0, y:0};
        let mut max_pos = Position{x:0, y:0};

        while i <= until {
            let next = Position{
                x:current.x + direction.x,
                y:current.y + direction.y,
            };

            if next.x > max_pos.x {
                grid.size.x += 1;  // adding a col on the right
                direction = Position::up();  // we were going right, now we go up
            } else if next.y > max_pos.y {
                grid.size.y += 1;  // adding a row up
                direction = Position::left();  // we were going up now we go left
            } else if next.x < min_pos.x {
                grid.size.x += 1;  // adding a col on the left
                direction = Position::down();  // we were going left, now we go down
            } else if next.y < min_pos.y {
                grid.size.y += 1;  // adding a row down
                direction = Position::right();  // we were going down, now we go right
            }

            min_pos = min_pos.min(&next);
            max_pos = max_pos.max(&next);
            current = next.clone();
            grid.grid.push(next);

            i += 1;
        }

        grid
    }
}

fn main() {
    let input: u32 = 347991;
    let grid = Grid::new(input);
    println!("Created a grid of {}", grid.size);
    let o = grid.get(1).unwrap();
    let t = grid.get(input).unwrap();
    println!("Distance between {} and 1 is {}",
        input,
        t.manhattan_distance(&o),
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected_grid, expected_size,
    case(1,
         &vec!(Position::new(0, 0)),
         &Position::new(1, 1),
    ),  // first step
    case(2,
         &vec!(Position::new(0, 0), Position::new(1, 0)),
         &Position::new(2, 1),
    ),  // going right
    case(3,
         &vec!(Position::new(0, 0), Position::new(1, 0), Position::new(1, 1)),
         &Position::new(2, 2),
    ),  // going up
    case(5,
         &vec!(Position::new(0, 0), Position::new(1, 0), Position::new(1, 1),
               Position::new(0, 1), Position::new(-1, 1)),
         &Position::new(3, 2),
    ),  // going left
    case(7,
         &vec!(Position::new(0, 0), Position::new(1, 0), Position::new(1, 1),
               Position::new(0, 1), Position::new(-1, 1), Position::new(-1, 0),
               Position::new(-1, -1)),
         &Position::new(3, 3),
    ),  // going down
    case(8,
         &vec!(Position::new(0, 0), Position::new(1, 0), Position::new(1, 1),
               Position::new(0, 1), Position::new(-1, 1), Position::new(-1, 0),
               Position::new(-1, -1), Position::new(0, -1)),
         &Position::new(3, 3),
    ),  // going right
    )]
    fn test_grid_new(input: u32, expected_grid: &Vec<Position>, expected_size: &Position) {
        let grid = Grid::new(input);
        println!("{:?}", grid.grid);
        assert!(grid.grid.eq(expected_grid));
        assert_eq!(grid.size, *expected_size);
    }
}
