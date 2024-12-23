use regex::Regex;
use std::{cmp::Reverse, collections::BinaryHeap, fs::read_to_string};

#[derive(Debug)]
struct Reach {
    col: usize,
    row: usize,
    cost: u32,
}

fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new(); // -1 for wall, distance from start otherwise, initialized at i32::MAX
    const GRID_SIZE: usize = 71;
    const MAX_CORRUPTION: usize = 1024;

    for _r in 0..GRID_SIZE {
        let grid_row: Vec<i32> = vec![i32::MAX; GRID_SIZE + 1];
        grid.push(grid_row);
    }

    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let re_coords = Regex::new(r"(\d+),(\d+)").unwrap();

    for (row_num, coords_string) in input_data.lines().enumerate() {
        if row_num >= MAX_CORRUPTION {
            break;
        }
        let matches = re_coords.captures(coords_string).unwrap();
        let x = matches[1].parse::<usize>().unwrap();
        let y = matches[2].parse::<usize>().unwrap();
        grid[y][x] = -1;
    }

    let mut heap: BinaryHeap<Reverse<Reach>> = BinaryHeap::new();
    grid[0][0] = 0;
    heap.push(Reverse(Reach {
        col: 0,
        row: 0,
        cost: 0,
    }));

    let result = loop {
        let cheapest = heap.pop();
        match cheapest {
            None => panic!(),
            Some(space) => {
                let x = space.0.col;
                let y = space.0.row;
                let cost = space.0.cost;

                // seed up
                if y > 0 && grid[y - 1][x] == i32::MAX {
                    grid[y - 1][x] = cost as i32 + 1;
                    heap.push(Reverse(Reach {
                        col: x,
                        row: y - 1,
                        cost: cost + 1,
                    }));
                }

                if y < GRID_SIZE - 1 && grid[y + 1][x] == i32::MAX {
                    if y == GRID_SIZE - 2 && x == GRID_SIZE - 1 {
                        break cost + 1;
                    }
                    grid[y + 1][x] = cost as i32 + 1;
                    heap.push(Reverse(Reach {
                        col: x,
                        row: y + 1,
                        cost: cost + 1,
                    }));
                }

                if x > 0 && grid[y][x - 1] == i32::MAX {
                    grid[y][x - 1] = cost as i32 + 1;
                    heap.push(Reverse(Reach {
                        col: x - 1,
                        row: y,
                        cost: cost + 1,
                    }));
                }

                if x < GRID_SIZE - 1 && grid[y][x + 1] == i32::MAX {
                    if y == GRID_SIZE - 1 && x == GRID_SIZE - 2 {
                        break cost + 1;
                    }
                    grid[y][x + 1] = cost as i32 + 1;
                    heap.push(Reverse(Reach {
                        col: x + 1,
                        row: y,
                        cost: cost + 1,
                    }));
                }
            }
        }
    };
    println!("Result: {}", result);
}

impl Ord for Reach {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, |x: &Self, y: &Self| x.cost.cmp(&y.cost))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, |x: &Self, y: &Self| x.cost.cmp(&y.cost))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self.cost < min.cost {
            min
        } else if self.cost > max.cost {
            max
        } else {
            self
        }
    }
}

impl PartialOrd for Reach {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for Reach {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.row == other.row && self.col == other.col
    }
}

impl Eq for Reach {}
