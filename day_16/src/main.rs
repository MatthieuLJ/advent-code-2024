use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::u32;

#[derive(PartialEq)]
enum Heading {
    NORTH,
    EAST,
    WEST,
    SOUTH,
}

struct Progress {
    col: usize,
    row: usize,
    cost: u32,
    heading: Heading,
}

fn main() {
    let input_data = read_to_string("input.txt").expect("Cannot read input file");
    let mut maze: Vec<Vec<char>> = Vec::new();

    for l in input_data.lines() {
        let new_line: Vec<char> = l.chars().into_iter().collect();
        maze.push(new_line);
    }

    let mut heap: BinaryHeap<Reverse<Progress>> = BinaryHeap::new();
    let mut path: Vec<Vec<Progress>> = Vec::new();
    let mut from: Vec<Vec<HashSet<(usize, usize)>>> = Vec::new();

    for r in 0..maze.len() {
        let mut path_line: Vec<Progress> = Vec::new();
        let mut from_line: Vec<HashSet<(usize, usize)>> = Vec::new();
        for c in 0..maze[r].len() {
            let from_cell: HashSet<(usize, usize)> = HashSet::new();
            from_line.push(from_cell);
            if maze[r][c] == 'S' {
                let empty_path: Progress = Progress {
                    col: c,
                    row: r,
                    cost: 0,
                    heading: Heading::EAST,
                };
                path_line.push(empty_path);
                heap.push(Reverse(Progress {
                    col: c,
                    row: r,
                    cost: 0,
                    heading: Heading::EAST,
                }));
            } else {
                let empty_path: Progress = Progress {
                    col: c,
                    row: r,
                    cost: u32::MAX,
                    heading: Heading::NORTH,
                };
                path_line.push(empty_path);
            }
        }
        from.push(from_line);
        path.push(path_line);
    }

    let mut destination_e: (usize, usize) = (0, 0);
    let mut max_distance: u32 = u32::MAX;

    let result = loop {
        let cheapest = heap.pop();
        match cheapest {
            None => panic!(),
            Some(space) => {
                let x = space.0.col;
                let y = space.0.row;
                let cost = space.0.cost;
                let dir = &space.0.heading;

                if cost > max_distance {
                    break (
                        path[destination_e.1][destination_e.0].cost,
                        trace_source(destination_e, &from),
                    );
                }

                // check to the right
                if maze[y][x + 1] != '#' {
                    let new_cost = match dir {
                        Heading::EAST => cost + 1,
                        Heading::NORTH => cost + 1001,
                        Heading::SOUTH => cost + 1001,
                        Heading::WEST => cost + 2001,
                    };
                    if path[y][x + 1].cost != u32::MAX && new_cost == path[y][x + 1].cost + 1000 {
                        from[y][x + 1].insert((x, y));
                    }
                    if new_cost < path[y][x + 1].cost {
                        path[y][x + 1] = Progress {
                            col: x + 1,
                            row: y,
                            cost: new_cost,
                            heading: Heading::EAST,
                        };
                        heap.push(Reverse(Progress {
                            col: x + 1,
                            row: y,
                            cost: new_cost,
                            heading: Heading::EAST,
                        }));
                        from[y][x + 1].insert((x, y));
                    }

                    if maze[y][x + 1] == 'E' {
                        destination_e = (x + 1, y);
                        max_distance = new_cost;
                    }
                }
                // check above
                if maze[y - 1][x] != '#' {
                    let new_cost = match dir {
                        Heading::EAST => cost + 1001,
                        Heading::NORTH => cost + 1,
                        Heading::SOUTH => cost + 2001,
                        Heading::WEST => cost + 1001,
                    };
                    if path[y - 1][x].cost != u32::MAX && new_cost == path[y - 1][x].cost + 1000 {
                        from[y - 1][x].insert((x, y));
                    }
                    if new_cost < path[y - 1][x].cost {
                        path[y - 1][x] = Progress {
                            col: x,
                            row: y - 1,
                            cost: new_cost,
                            heading: Heading::NORTH,
                        };
                        heap.push(Reverse(Progress {
                            col: x,
                            row: y - 1,
                            cost: new_cost,
                            heading: Heading::NORTH,
                        }));
                        from[y - 1][x].insert((x, y));
                    }

                    if maze[y - 1][x] == 'E' {
                        destination_e = (x, y - 1);
                        max_distance = new_cost;
                    }
                }

                // check to the left
                if maze[y][x - 1] != '#' {
                    let new_cost = match dir {
                        Heading::EAST => cost + 2001,
                        Heading::NORTH => cost + 1001,
                        Heading::SOUTH => cost + 1001,
                        Heading::WEST => cost + 1,
                    };
                    if path[y][x - 1].cost != u32::MAX && new_cost == path[y][x - 1].cost + 1000 {
                        from[y][x - 1].insert((x, y));
                    }
                    if new_cost < path[y][x - 1].cost {
                        path[y][x - 1] = Progress {
                            col: x - 1,
                            row: y,
                            cost: new_cost,
                            heading: Heading::WEST,
                        };
                        heap.push(Reverse(Progress {
                            col: x - 1,
                            row: y,
                            cost: new_cost,
                            heading: Heading::WEST,
                        }));
                        from[y][x - 1].insert((x, y));
                    }

                    if maze[y][x - 1] == 'E' {
                        destination_e = (x - 1, y);
                        max_distance = new_cost;
                    }
                }

                // check below
                if maze[y + 1][x] != '#' {
                    let new_cost = match dir {
                        Heading::EAST => cost + 1001,
                        Heading::NORTH => cost + 2001,
                        Heading::SOUTH => cost + 1,
                        Heading::WEST => cost + 1001,
                    };
                    if path[y + 1][x].cost != u32::MAX && new_cost == path[y + 1][x].cost + 1000 {
                        from[y + 1][x].insert((x, y));
                    }
                    if new_cost < path[y + 1][x].cost {
                        path[y + 1][x] = Progress {
                            col: x,
                            row: y + 1,
                            cost: new_cost,
                            heading: Heading::SOUTH,
                        };
                        heap.push(Reverse(Progress {
                            col: x,
                            row: y + 1,
                            cost: new_cost,
                            heading: Heading::SOUTH,
                        }));
                        from[y + 1][x].insert((x, y));
                    }

                    if maze[y + 1][x] == 'E' {
                        destination_e = (x, y + 1);
                        max_distance = new_cost;
                    }
                }
            }
        }
    };

    println!("Result: {} with {} seats", result.0, result.1.len());
}

fn trace_source(
    at: (usize, usize),
    from: &Vec<Vec<HashSet<(usize, usize)>>>,
) -> HashSet<(usize, usize)> {
    let mut result: HashSet<(usize, usize)> = HashSet::new();
    let mut to_process : VecDeque<(usize,usize)> = VecDeque::new();

    to_process.push_back(at);

    while to_process.len() > 0 {
        let next_seat = to_process.pop_front().unwrap();
        if !result.contains(&next_seat) {
            result.insert(next_seat);
            for n in &from[next_seat.1][next_seat.0] {
                to_process.push_back(*n);
            }
        }
    }
    result
}

impl Ord for Progress {
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

impl PartialOrd for Progress {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for Progress {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.heading == other.heading
    }
}

impl Eq for Progress {}
