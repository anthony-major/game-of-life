use std::{collections::HashSet, hash::Hash};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: i32,
    pub column: i32,
}

impl Coord {
    pub fn new(row: i32, column: i32) -> Self {
        Self {
            row: row,
            column: column,
        }
    }
}

pub struct Board {
    cells: HashSet<Coord>,
    generation: u32,
}

impl Board {
    pub fn new(seed: Vec<Coord>) -> Self {
        let mut cells: HashSet<Coord> = HashSet::new();

        for cell in seed {
            cells.insert(cell);
        }

        Self {
            cells: cells,
            generation: 0,
        }
    }

    pub fn new_random(width: u32, height: u32) -> Self {
        let seed: Vec<Coord> = (0..width * height)
            .filter_map(|i| {
                if rand::random::<f32>() < 0.1 {
                    Some(Coord::new((i / height) as i32, (i % width) as i32))
                } else {
                    None
                }
            })
            .collect();

        Self::new(seed)
    }

    pub fn cells(&self) -> &HashSet<Coord> {
        &self.cells
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn population(&self) -> u32 {
        self.cells.len() as u32
    }

    pub fn update(&mut self) {
        let mut new_cells: HashSet<Coord> = HashSet::new();
        let mut checked_cells: HashSet<Coord> = HashSet::new();

        for cell in &self.cells {
            let neighbors = [
                Coord::new(cell.row - 1, cell.column - 1),
                Coord::new(cell.row - 1, cell.column),
                Coord::new(cell.row - 1, cell.column + 1),
                Coord::new(cell.row, cell.column - 1),
                Coord::new(cell.row, cell.column + 1),
                Coord::new(cell.row + 1, cell.column - 1),
                Coord::new(cell.row + 1, cell.column),
                Coord::new(cell.row + 1, cell.column + 1),
                cell.clone(),
            ];

            for neighbor in &neighbors {
                if checked_cells.contains(neighbor) {
                    continue;
                }

                let live_neighbor_count = self.get_live_neighbors_count(neighbor);

                if (live_neighbor_count == 2 && self.cells.contains(neighbor))
                    || live_neighbor_count == 3
                {
                    new_cells.insert(neighbor.clone());
                }

                checked_cells.insert(neighbor.clone());
            }
        }

        self.cells = new_cells;
        self.generation += 1;
    }

    fn get_live_neighbors_count(&self, cell: &Coord) -> u32 {
        let mut count = 0u32;

        let top_left = self
            .cells
            .contains(&Coord::new(cell.row - 1, cell.column - 1));
        let top = self.cells.contains(&Coord::new(cell.row - 1, cell.column));
        let top_right = self
            .cells
            .contains(&Coord::new(cell.row - 1, cell.column + 1));
        let left = self.cells.contains(&Coord::new(cell.row, cell.column - 1));
        let right = self.cells.contains(&Coord::new(cell.row, cell.column + 1));
        let bottom_left = self
            .cells
            .contains(&Coord::new(cell.row + 1, cell.column - 1));
        let bottom = self.cells.contains(&Coord::new(cell.row + 1, cell.column));
        let bottom_right = self
            .cells
            .contains(&Coord::new(cell.row + 1, cell.column + 1));

        if top_left {
            count += 1;
        }
        if top {
            count += 1;
        }
        if top_right {
            count += 1;
        }
        if left {
            count += 1;
        }
        if right {
            count += 1;
        }
        if bottom_left {
            count += 1;
        }
        if bottom {
            count += 1;
        }
        if bottom_right {
            count += 1;
        }

        count
    }
}
