use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
    fs::read_to_string,
};

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_coords = (0, 0);
    let mut end_coords = (0, 0);

    let mut distance_from_start: Vec<Vec<u32>> = Vec::new();
    let mut distance_from_end: Vec<Vec<u32>> = Vec::new();

    for (row, l) in input_data.lines().enumerate() {
        let new_row: Vec<char> = l.chars().collect();
        grid.push(new_row);
        if let Some(col) = l.find("S") {
            start_coords = (col, row);
        } else if let Some(col) = l.find("E") {
            end_coords = (col, row);
        }

        let new_row_from_start = vec![u32::MAX; l.len()];
        distance_from_start.push(new_row_from_start);
        let new_row_from_end = vec![u32::MAX; l.len()];
        distance_from_end.push(new_row_from_end);
    }

    fill_djikstra(&grid, start_coords, &mut distance_from_start);
    fill_djikstra(&grid, end_coords, &mut distance_from_end);

    let total_distance = distance_from_start[end_coords.1][end_coords.0];
    let mut result: u32 = 0;

    for (row, l) in grid.clone().into_iter().enumerate() {
        if row == 0 || row == grid.len() - 1 {
            continue;
        }
        for (col, c) in l.into_iter().enumerate() {
            if col == 0 || col == grid[row].len() - 1 {
                continue;
            }
            if c == '#' {
                // what if we removed this wall piece
                let shortest_to_start = min(
                    min(
                        min(
                            distance_from_start[row - 1][col],
                            distance_from_start[row][col + 1],
                        ),
                        distance_from_start[row + 1][col],
                    ),
                    distance_from_start[row][col - 1],
                );
                let shortest_to_end = min(
                    min(
                        min(
                            distance_from_end[row - 1][col],
                            distance_from_end[row][col + 1],
                        ),
                        distance_from_end[row + 1][col],
                    ),
                    distance_from_end[row][col - 1],
                );
                if shortest_to_end < u32::MAX
                    && shortest_to_start < u32::MAX
                    && shortest_to_end + shortest_to_start + 2 < total_distance
                {
                    let save = total_distance - (shortest_to_end + shortest_to_start + 2);
                    println!("This one saves {}", save);
                    if save >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Reach {
    col: usize,
    row: usize,
    cost: u32,
}

fn fill_djikstra(grid: &Vec<Vec<char>>, origin: (usize, usize), cost_map: &mut Vec<Vec<u32>>) {
    let mut heap: BinaryHeap<Reverse<Reach>> = BinaryHeap::new();
    cost_map[origin.1][origin.0] = 0;
    heap.push(Reverse(Reach {
        col: origin.0,
        row: origin.1,
        cost: 0,
    }));

    loop {
        let cheapest = heap.pop();
        match cheapest {
            None => {
                break;
            }
            Some(space) => {
                let x = space.0.col;
                let y = space.0.row;
                let cost = space.0.cost;

                // seed up
                if grid[y - 1][x] != '#' && cost_map[y - 1][x] == u32::MAX {
                    cost_map[y - 1][x] = cost + 1;
                    heap.push(Reverse(Reach {
                        col: x,
                        row: y - 1,
                        cost: cost + 1,
                    }));
                }

                if grid[y + 1][x] != '#' && cost_map[y + 1][x] == u32::MAX {
                    cost_map[y + 1][x] = cost + 1;
                    heap.push(Reverse(Reach {
                        col: x,
                        row: y + 1,
                        cost: cost + 1,
                    }));
                }

                if grid[y][x - 1] != '#' && cost_map[y][x - 1] == u32::MAX {
                    cost_map[y][x - 1] = cost + 1;
                    heap.push(Reverse(Reach {
                        col: x - 1,
                        row: y,
                        cost: cost + 1,
                    }));
                }

                if grid[y][x + 1] != '#' && cost_map[y][x + 1] == u32::MAX {
                    cost_map[y][x + 1] = cost + 1;
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
