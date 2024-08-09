pub struct Coord {
    row: usize,
    column: usize,
}

impl Coord {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row: row,
            column: column,
        }
    }
}

pub struct Board {
    cells: Vec<bool>,
    width: usize,
    height: usize,
    generation: usize,
    population: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![false; width * height],
            width: width,
            height: height,
            generation: 0,
            population: 0,
        }
    }

    pub fn seed(&mut self, coords: Vec<Coord>) {
        for coord in coords {
            let i = self.get_index(coord.row, coord.column);

            // If the cell we are setting is currently dead, then there will be a population increase.
            if !self.cells[i] {
                self.population += 1;
            }

            self.cells[i] = true;
        }
    }

    pub fn cells(&self) -> &Vec<bool> {
        &self.cells
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
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

        for row in 0..self.height {
            for column in 0..self.width {
                let live_neighbors_count = self.get_live_neighbors_count(row, column);
                let i = self.get_index(row, column);

                match live_neighbors_count {
                    x if x < 2 => new_cells[i] = false,
                    x if x > 3 => new_cells[i] = false,
                    x if x == 3 => new_cells[i] = true,
                    _ => {}
                }

                if new_cells[i] {
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
                    || neighbor_row >= (self.height as isize)
                    || neighbor_column < 0
                    || neighbor_column >= (self.width as isize)
                {
                    continue;
                }

                let i_cell = self.get_index(neighbor_row as usize, neighbor_column as usize);

                if self.cells[i_cell] {
                    live_neighbor_count += 1;
                }
            }
        }

        live_neighbor_count
    }
}
