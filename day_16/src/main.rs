use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

#[derive(PartialEq, Clone, Copy)]
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
    trail: HashSet<(usize, usize)>,
}

fn main() {
    let input_data = read_to_string("input_test.txt").expect("Cannot read input file");
    let mut maze: Vec<Vec<char>> = Vec::new();

    for l in input_data.lines() {
        let new_line: Vec<char> = l.chars().collect();
        maze.push(new_line);
    }

    let mut heap: BinaryHeap<Reverse<Progress>> = BinaryHeap::new();
    let mut path: Vec<Vec<Vec<Progress>>> = Vec::new();

    for r in 0..maze.len() {
        let mut path_line: Vec<Vec<Progress>> = Vec::new();
        for c in 0..maze[r].len() {
            let mut path_cell: Vec<Progress> = Vec::new();
            if maze[r][c] == 'S' {
                path_cell.push(Progress {
                    col: c,
                    row: r,
                    cost: 0,
                    heading: Heading::EAST,
                    trail: HashSet::from([(c, r)]),
                });
                heap.push(Reverse(Progress {
                    col: c,
                    row: r,
                    cost: 0,
                    heading: Heading::EAST,
                    trail: HashSet::from([(c, r)]),
                }));
            } else {
                let empty_path: Progress = Progress {
                    col: c,
                    row: r,
                    cost: u32::MAX,
                    heading: Heading::NORTH,
                    trail: HashSet::new(),
                };
                path_cell.push(empty_path);
            }
            path_line.push(path_cell);
        }
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
                let trail = &space.0.trail;

                if cost > max_distance {
                    let mut min_cost = u32::MAX;
                    let mut min_cost_index: usize = usize::MAX;
                    for i in 0..path[destination_e.1][destination_e.0].len() {
                        if path[destination_e.1][destination_e.0][i].cost < min_cost {
                            min_cost = path[destination_e.1][destination_e.0][i].cost;
                            min_cost_index = i;
                        }
                    }
                    break (
                        min_cost,
                        path[destination_e.1][destination_e.0][min_cost_index]
                            .trail
                            .len(),
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
                    let mut new_trail = trail.clone();
                    new_trail.insert((x + 1, y));

                    add_progress((x + 1, y), new_cost, &Heading::EAST, &new_trail, &mut path);
                    heap.push(Reverse(Progress {
                        col: x + 1,
                        row: y,
                        cost: new_cost,
                        trail: new_trail.clone(),
                        heading: Heading::EAST,
                    }));

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
                    let mut new_trail = trail.clone();
                    new_trail.insert((x, y - 1));

                    add_progress((x, y - 1), new_cost, &Heading::SOUTH, &new_trail, &mut path);
                    heap.push(Reverse(Progress {
                        col: x,
                        row: y - 1,
                        cost: new_cost,
                        trail: new_trail.clone(),
                        heading: Heading::NORTH,
                    }));

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
                    let mut new_trail = trail.clone();
                    new_trail.insert((x - 1, y));

                    add_progress((x - 1, y), new_cost, &Heading::WEST, &new_trail, &mut path);
                    heap.push(Reverse(Progress {
                        col: x - 1,
                        row: y,
                        cost: new_cost,
                        trail: new_trail.clone(),
                        heading: Heading::WEST,
                    }));

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
                    let mut new_trail = trail.clone();
                    new_trail.insert((x, y + 1));

                    add_progress((x, y + 1), new_cost, &Heading::SOUTH, &new_trail, &mut path);

                    heap.push(Reverse(Progress {
                        col: x,
                        row: y + 1,
                        cost: new_cost,
                        trail: new_trail.clone(),
                        heading: Heading::SOUTH,
                    }));

                    if maze[y + 1][x] == 'E' {
                        destination_e = (x, y + 1);
                        max_distance = new_cost;
                    }
                }
            }
        }
    };

    println!("Result: {} with {} seats", result.0, result.1);
}

fn add_progress(
    at: (usize, usize),
    cost: u32,
    heading: &Heading,
    trail: &HashSet<(usize, usize)>,
    path: &mut Vec<Vec<Vec<Progress>>>,
) {
    for i in 0..path[at.1][at.0].len() {
        if path[at.1][at.0][i].heading == *heading {
            // now compare and pick the one with the cheapest "cost" then return
            if path[at.1][at.0][i].cost < cost {
                return;
            } else if path[at.1][at.0][i].cost == cost {
                // if equal, add the trail
                path[at.1][at.0][i].trail.extend(trail);
            } else {
                // replace with the new progress
                path[at.1][at.0][i] = Progress {
                    col: at.0,
                    row: at.1,
                    cost,
                    heading: heading.clone(),
                    trail: trail.clone(),
                };
            }
        }
    }
    path[at.1][at.0].push(Progress {
        col: at.0,
        row: at.1,
        cost,
        heading: heading.clone(),
        trail: trail.clone(),
    });
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
