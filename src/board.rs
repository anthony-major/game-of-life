pub type Coord = (usize, usize);

pub struct Board {
    cells: Vec<Vec<bool>>,
    generation: usize,
    population: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![false; width]; height],
            generation: 0,
            population: 0,
        }
    }

    pub fn seed(&mut self, coords: Vec<Coord>) {
        for coord in coords {
            // If the cell we are setting is currently dead, then there will be a population increase.
            if !self.cells[coord.1][coord.0] {
                self.population += 1;
            }

            self.cells[coord.1][coord.0] = true;
        }
    }

    pub fn cells(&self) -> &Vec<Vec<bool>> {
        &self.cells
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn population(&self) -> usize {
        self.population
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();
        let mut new_population = 0usize;

        for row in 0..self.cells.len() {
            for column in 0..self.cells[row].len() {
                let live_neighbors_count = self.get_live_neighbors_count(row, column);

                match live_neighbors_count {
                    x if x < 2 => new_cells[row][column] = false,
                    x if x > 3 => new_cells[row][column] = false,
                    x if x == 3 => new_cells[row][column] = true,
                    _ => {}
                }

                if new_cells[row][column] {
                    new_population += 1;
                }
            }
        }

        self.cells = new_cells;
        self.generation += 1;
        self.population = new_population;
    }

    fn get_live_neighbors_count(&self, row: usize, column: usize) -> usize {
        let mut live_neighbor_count = 0usize;

        for i in -1isize..=1 {
            for j in -1isize..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor_row = (row as isize) + i;
                let neighbor_column = (column as isize) + j;

                if neighbor_row < 0
                    || neighbor_row >= (self.cells.len() as isize)
                    || neighbor_column < 0
                    || neighbor_column >= (self.cells[row].len() as isize)
                {
                    continue;
                }

                if self.cells[neighbor_row as usize][neighbor_column as usize] {
                    live_neighbor_count += 1;
                }
            }
        }

        live_neighbor_count
    }
}
