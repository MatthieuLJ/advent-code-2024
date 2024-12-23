use regex::Regex;
use std::{cmp::Reverse, collections::BinaryHeap, fs::read_to_string};

#[derive(Debug)]
struct Reach {
    col: usize,
    row: usize,
    cost: u32,
}

const GRID_SIZE: usize = 71;
const MAX_CORRUPTION: usize = 1024;

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let total_lines = input_data.lines().collect::<Vec<&str>>().len();
    let re_coords = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut corruptions: Vec<(usize, usize)> = Vec::new();

    for coords_string in input_data.lines() {
        let matches = re_coords.captures(coords_string).unwrap();
        let x = matches[1].parse::<usize>().unwrap();
        let y = matches[2].parse::<usize>().unwrap();
        corruptions.push((x, y));
    }

    let mut min_time = MAX_CORRUPTION;
    let mut max_time = total_lines;
    while max_time > min_time + 1 {
        let mid_time = min_time + (max_time - min_time) / 2;
        let mut grid: Vec<Vec<i32>> = Vec::new(); // -1 for wall, distance from start otherwise, initialized at i32::MAX

        for _r in 0..GRID_SIZE {
            let grid_row: Vec<i32> = vec![i32::MAX; GRID_SIZE + 1];
            grid.push(grid_row);
        }

        for i in 0..mid_time {
            grid[corruptions[i].1][corruptions[i].0] = -1;
        }
        let found = find_path(&mut grid);
        match found {
         None => {max_time = mid_time; println!("Less than {}", mid_time);},
         Some(x) => {min_time = mid_time; println!("More than {} (found a path in {})", mid_time,x);},
        }
    }
    println!("{:?}", corruptions[min_time]);

}

fn find_path(grid: &mut Vec<Vec<i32>>) -> Option<u32> {
    let mut heap: BinaryHeap<Reverse<Reach>> = BinaryHeap::new();
    grid[0][0] = 0;
    heap.push(Reverse(Reach {
        col: 0,
        row: 0,
        cost: 0,
    }));

    loop {
        let cheapest = heap.pop();
        match cheapest {
            None => {
                break None;
            }
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
                        break Some(cost + 1);
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
                        break Some(cost + 1);
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
    }
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
