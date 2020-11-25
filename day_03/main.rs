use std::cmp;
use core::fmt;

#[derive(Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

trait PositionTrait {
    fn new(x: i32, y: i32) -> Self;
    fn manhattan_distance(self, other: &Self) -> u32;
    fn min(&self, other: &Self) -> Position;
    fn max(&self, other: &Self) -> Position;
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position {x, y}
    }

    fn adjacent(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn manhattan_distance(self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
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

    fn up() -> Position {
        Position::new(0, 1)
    }

    fn down() -> Position {
        Position::new(0, -1)
    }

    fn right() -> Position {
        Position::new(1, 0)
    }

    fn left() -> Position {
        Position::new(-1, 0)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Cell {
    position: Position,
    value: u64,
}

impl Cell {
    fn new(position: Position, value: u64) -> Self {
        Cell{
            position,
            value,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} [{}]", self.position, self.value)
    }
}

struct Grid {
    grid: Vec<Cell>,
    size: Position,
}

impl Grid {
    fn empty() -> Grid {
        Grid{
            grid: vec!(Cell::new(Position::new(0, 0), 1)),
            size: Position{x:1, y:1},
        }
    }

    fn get(&self, i: u32) -> Option<Cell> {
        let d = self.grid.get((i - 1) as usize);
        if d.is_none() {
            None
        } else {
            Some((*d.unwrap()).clone())
        }
    }

    fn sum_adjacent(&self, position: &Position) -> u64 {
        let mut sum = 0;
        for cell in &self.grid {
            if cell.position.adjacent(position) {
                eprintln!("Adding {} to {}", cell.value, sum);
                sum += cell.value;
            }
        }
        sum
    }

    fn new(until: u32) -> Grid {
        let mut grid = Grid::empty();
        let mut i: u32 = 2;
        let mut current= grid.grid[0].position.clone();
        let mut direction = Position::right();

        let mut min_pos = Position{x:0, y:0};
        let mut max_pos = Position{x:0, y:0};
        let mut found = false;

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
            // I have to do this because for large enough numbers we overflow a u64...
            // because I have a bug or it's a trap.
            let value = if !found {
                let value = grid.sum_adjacent(&next);

                if value > until as u64 {
                    println!("First value > {} is {}", until, value);
                    found = true;
                }
                value
            } else {
                0
            };

            grid.grid.push(Cell::new(next, value));

            i += 1;
        }

        grid
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input: u32 = if args.len() > 0 {
        args[1].parse().expect("not an integer")
    } else {
        347991
    };
    let grid = Grid::new(input);
    println!("Created a grid of {:?}", grid.size);
    let o = grid.get(1).unwrap();
    let t = grid.get(input).unwrap();
    println!("Distance between {} and 1 is {}",
        input,
        t.position.manhattan_distance(&o.position),
    );

    // for cell in &grid.grid {
    //     if cell.value > input as u64 {
    //         println!("First value > {} is {}", input, cell.value);
    //         break;
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    impl Cell {
        fn make(x: i32, y: i32, value: u64) -> Self {
            // To help when we don't care much about the value
            Cell {
                position: Position{x, y},
                value,
            }
        }
    }

    #[rstest(input, expected_grid, expected_size,
    case(1,
         &vec!(Cell::make(0, 0, 1)),
         &Position::new(1, 1),
    ),  // first step
    case(2,
         &vec!(Cell::make(0, 0, 1), Cell::make(1, 0, 1)),
         &Position::new(2, 1),
    ),  // going right
    case(3,
         &vec!(Cell::make(0, 0, 1), Cell::make(1, 0, 1), Cell::make(1, 1, 2)),
         &Position::new(2, 2),
    ),  // going up
    case(5,
         &vec!(Cell::make(0, 0, 1), Cell::make(1, 0, 1), Cell::make(1, 1, 2),
               Cell::make(0, 1, 4), Cell::make(-1, 1, 5)),
         &Position::new(3, 2),
    ),  // going left
    case(7,
         &vec!(Cell::make(0, 0, 1), Cell::make(1, 0, 1), Cell::make(1, 1, 2),
               Cell::make(0, 1, 4), Cell::make(-1, 1, 5), Cell::make(-1, 0, 10),
               Cell::make(-1, -1, 0)),  // 11, but > 7
         &Position::new(3, 3),
    ),  // going down
    case(8,
         &vec!(Cell::make(0, 0, 1), Cell::make(1, 0, 1), Cell::make(1, 1, 2),
               Cell::make(0, 1, 4), Cell::make(-1, 1, 5), Cell::make(-1, 0, 10),
               Cell::make(-1, -1, 0), Cell::make(0, -1, 0)),  // 11 and 23 but > 8
         &Position::new(3, 3),
    ),  // going right
    )]
    fn test_grid_new(input: u32, expected_grid: &Vec<Cell>, expected_size: &Position) {
        let grid = Grid::new(input);
        println!("{:?}", grid.grid);
        assert!(grid.grid.eq(expected_grid));
        assert_eq!(grid.size, *expected_size);
    }

    #[rstest(a, b, expected,
    case(&Position::new(0, 0), &Position::new(0, 0), true),  // itself
    case(&Position::new(0, 0), &Position::new(0, 1), true),
    case(&Position::new(0, 0), &Position::new(1, 0), true),
    case(&Position::new(0, 0), &Position::new(0, -1), true),
    case(&Position::new(0, 0), &Position::new(-1, 0), true),
    case(&Position::new(0, 0), &Position::new(1, 1), true),
    case(&Position::new(0, 0), &Position::new(-1, -1), true),  // diagonal too
    case(&Position::new(0, 0), &Position::new(0, 2), false),
    )]
    fn test_adjascent(a: &Position, b: &Position, expected: bool) {
        assert!(a.adjacent(b) == expected);
    }
}
