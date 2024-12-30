use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

const TUNNEL_LENGTH: isize = 20;

fn main() {
    let input_data = read_to_string("input_test.txt").expect("Cannot read input file");
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
    let mut cheats: HashMap<(usize, usize, usize, usize), u32> = HashMap::new();

    for (row, l) in grid.clone().into_iter().enumerate() {
        //if row == 0 || row == grid.len() - 1 {
        //    continue;
        //}
        for (col, c) in l.into_iter().enumerate() {
            //if col == 0 || col == grid[row].len() - 1 {
            //    continue;
            //}
            if c == '#' {
                // start the tunnel from there
                let mut shortest_to_start = u32::MAX;
                let mut shortest_start_point: (usize, usize) = (0, 0);

                if row > 0 && distance_from_start[row - 1][col] < shortest_to_start {
                    shortest_start_point = (col, row - 1);
                    shortest_to_start = distance_from_start[row - 1][col];
                }
                if row < grid.len() - 1 && distance_from_start[row + 1][col] < shortest_to_start {
                    shortest_start_point = (col, row + 1);
                    shortest_to_start = distance_from_start[row + 1][col];
                }
                if col > 0 && distance_from_start[row][col - 1] < shortest_to_start {
                    shortest_start_point = (col - 1, row);
                    shortest_to_start = distance_from_start[row][col - 1];
                }
                if col < grid[row].len() - 1
                    && distance_from_start[row][col + 1] < shortest_to_start
                {
                    shortest_start_point = (col + 1, row);
                    shortest_to_start = distance_from_start[row][col + 1];
                }

                if shortest_to_start == u32::MAX {
                    continue;
                }

                let mut heap: BinaryHeap<Reverse<Reach>> = BinaryHeap::new();
                heap.push(Reverse(Reach { col, row, cost: 1 }));
                let mut visited: Vec<Vec<bool>> = Vec::new();
                for another_row in 0..grid.len() {
                    let not_visited: Vec<bool> = vec![false; grid[another_row].len()];
                    visited.push(not_visited);
                }
                visited[row][col] = true;
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

                            // look up
                            if y >= 1 && !visited[y - 1][x] {
                                if cost < TUNNEL_LENGTH as u32 {
                                    heap.push(Reverse(Reach {
                                        col: x,
                                        row: y - 1,
                                        cost: cost + 1,
                                    }));
                                }
                                if grid[y - 1][x] != '#' {
                                    check_on_path(
                                        &distance_from_end,
                                        x,
                                        y - 1,
                                        shortest_to_start,
                                        cost,
                                        total_distance,
                                        &mut cheats,
                                        shortest_start_point,
                                    );
                                }
                                visited[y - 1][x] = true;
                            }
                            // look down
                            if y <= grid.len() - 2 && !visited[y + 1][x] {
                                if cost < TUNNEL_LENGTH as u32 {
                                    heap.push(Reverse(Reach {
                                        col: x,
                                        row: y + 1,
                                        cost: cost + 1,
                                    }));
                                }
                                if grid[y + 1][x] != '#' {
                                    check_on_path(
                                        &distance_from_end,
                                        x,
                                        y + 1,
                                        shortest_to_start,
                                        cost,
                                        total_distance,
                                        &mut cheats,
                                        shortest_start_point,
                                    );
                                }
                                visited[y + 1][x] = true;
                            }
                            // look right
                            if x <= grid[0].len() - 2 && !visited[y][x + 1] {
                                if cost < TUNNEL_LENGTH as u32 {
                                    heap.push(Reverse(Reach {
                                        col: x + 1,
                                        row: y,
                                        cost: cost + 1,
                                    }));
                                }
                                if grid[y][x + 1] != '#' {
                                    check_on_path(
                                        &distance_from_end,
                                        x + 1,
                                        y,
                                        shortest_to_start,
                                        cost,
                                        total_distance,
                                        &mut cheats,
                                        shortest_start_point,
                                    );
                                }
                                visited[y][x + 1] = true;
                            }
                            // look left
                            if x >= 1 && !visited[y][x - 1] {
                                if cost < TUNNEL_LENGTH as u32 {
                                    heap.push(Reverse(Reach {
                                        col: x - 1,
                                        row: y,
                                        cost: cost + 1,
                                    }));
                                }
                                if grid[y][x - 1] != '#' {
                                    check_on_path(
                                        &distance_from_end,
                                        x - 1,
                                        y,
                                        shortest_to_start,
                                        cost,
                                        total_distance,
                                        &mut cheats,
                                        shortest_start_point,
                                    );
                                }
                                visited[y][x - 1] = true;
                            }
                        }
                    }
                }
            }
        }
        for (k, v) in cheats.clone() {
            if v > 75 {
                println!("Saved {} from {},{} to {},{}", v, k.0, k.1, k.2, k.3);
            }
        }
    }
}

fn check_on_path(
    distance_from_end: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    shortest_to_start: u32,
    cost: u32,
    total_distance: u32,
    cheats: &mut HashMap<(usize, usize, usize, usize), u32>,
    shortest_start_point: (usize, usize),
) {
    if total_distance > shortest_to_start + distance_from_end[y][x] + cost + 1 {
        let save = total_distance - (shortest_to_start + distance_from_end[y][x] + cost + 1);
        cheats
            .entry((shortest_start_point.0, shortest_start_point.1, x, y))
            .and_modify(|s| *s = max(*s, save))
            .or_insert(save);
    }
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
