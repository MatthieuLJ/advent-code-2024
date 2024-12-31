use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

const TUNNEL_LENGTH: u32 = 20;

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
    let mut cheats: HashMap<(usize, usize, usize, usize), u32> = HashMap::new();

    for (row_start, l) in grid.clone().into_iter().enumerate() {
        for (col_start, c) in l.into_iter().enumerate() {
            if c == '#' {
                continue;
            }
            // scan for the end of the tunnel
            for row_end in 0..grid.len() {
                for col_end in 0..grid[row_end].len() {
                    if grid[row_end][col_end] == '#' {
                        continue;
                    }

                    // calculate the saving
                    let cheat_length = ((col_end as isize - col_start as isize).abs()
                        + (row_end as isize - row_start as isize).abs())
                        as u32;

                    if cheat_length > TUNNEL_LENGTH {
                        continue;
                    }

                    let new_distance = distance_from_start[row_start][col_start]
                        + cheat_length
                        + distance_from_end[row_end][col_end];

                    if new_distance > total_distance {
                        continue;
                    }

                    cheats.insert((col_start, row_start, col_end, row_end), total_distance - new_distance);
                }
            }
        }
    }
    /*
    let mut result: HashMap<u32, u32> = HashMap::new();
    for (k, v) in &cheats {
        result
            .entry(*v)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if *v == 70 {
            println!("Saved 72 with route from ({},{}) to ({},{})", k.1, k.0, k.3, k.2);
        }
    }
    for s in 50..=76 {
        println!("Saved {} with {} routes", s, result.get(&s).unwrap_or(&0));
    }
    */
    let mut result: u32 = 0;
    for (_k, v) in &cheats {
        if *v >= 100 {
            result += 1;
        }
    }
    println!("Found {} paths saving more than 100 ps", result);
    
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
