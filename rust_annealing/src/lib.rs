use wasm_bindgen::prelude::*;
use rand::Rng;

const NATURAL_E: f64 = std::f64::consts::E;

struct Cell {
    // The x position of the cell is 0-indexed
    x: i32,
    // The y position of the cell is 0-indexed and starts from the top of the grid
    y: i32,
    // The state of the cell should be "U" for up or "D" for down
    state: char
}

impl Cell {
    fn flip(&mut self) {
        if self.state == 'U' {
            self.state = 'D'
        } else if self.state == 'D' {
            self.state = 'U'
        } else {
            panic!("{}",self.state)
        }
    }
}

// Define the Grid struct
#[wasm_bindgen]
pub struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    pairing_energy: f64,
    temperature: f64,
    external_field: f64, // Positive values lower energy for the "U" state
}

#[wasm_bindgen]
impl Grid {

    pub fn new() -> Grid {
        let mut grid = Grid {
            width: 10,
            height: 10,
            cells: Vec::new(),
            pairing_energy: -1.0,
            temperature: 0.0,
            external_field: 0.05,
            };
        grid.fill_cells();
        grid
    }

    fn fill_cells(&mut self) {
        for i in 0..self.width {
            for ii in 0..self.height {
                self.add_cell(i,ii);
            }
        }
    }

    fn add_cell(&mut self, x: i32, y: i32) {
        let random_number = rand::thread_rng().gen_range(1..=2);
        let state = match random_number {
            1 => {'U'}
            2 => {'D'}
            i32::MIN..=0_i32 | 3_i32..=i32::MAX => panic!("Non-1 or 2 state produced")
        };        
        self.cells.push(Cell {
            x: x,
            y: y,
            state: state
        });
    }

    fn score(&self) -> f64 {
        let mut score: f64 = 0.;
        for cell1 in &self.cells {
            let mut matching_cells: Vec<&Cell> = Vec::new();
            for cell2 in &self.cells {
                if (cell1.x - cell2.x).abs() == 1 && (cell1.y - cell2.y).abs() == 0 {
                    matching_cells.push(cell2)
                } else if (cell1.x - cell2.x).abs() == 0 && (cell1.y - cell2.y).abs() == 1 {
                    matching_cells.push(cell2)
                } else {
                    continue
                }
            }

            for cell2 in matching_cells {
                if cell1.state == cell2.state {
                    score = score + self.pairing_energy
                }
            }

            if cell1.state == 'U' {
                score = score - self.external_field
            } else {
                score = score + self.external_field
            }
        }
        score
    }

    fn mutate(&mut self,mutation_index: usize) {
        self.cells[mutation_index].flip();
    }

    fn score_mutation(&mut self,mutation_index: usize) -> f64 {
        self.mutate(mutation_index);
        let score = self.score();
        self.mutate(mutation_index);
        score
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn return_temperature(&self) -> f64 {
        self.temperature
    }

    pub fn set_pairing_energy(&mut self, pairing_energy: f64) {
        self.pairing_energy = pairing_energy;
    }

    pub fn return_pairing_energy(&self) -> f64 {
        self.pairing_energy
    }

    pub fn set_external_field(&mut self, external_field: f64) {
        self.external_field = external_field;
    }

    pub fn return_external_field(&self) -> f64 {
        self.external_field
    }

    pub fn advance(&mut self) -> f64 {
        let old_energy = self.score();
        let mutation_index: usize = rand::thread_rng().gen_range(1..self.cells.len());
        let new_energy = self.score_mutation(mutation_index);

        let mut final_energy = old_energy;
        if new_energy < old_energy {
            self.mutate(mutation_index);
            final_energy = new_energy;
        } else {
            let acceptance_probability: f64 = NATURAL_E.powf((old_energy-new_energy)/self.temperature);
            let acceptance_draw: f64 = rand::thread_rng().gen();
            if acceptance_draw < acceptance_probability {
                self.mutate(mutation_index);
                final_energy = new_energy;
            }
        }
        final_energy
    }

    pub fn return_state(&self) -> Vec<i32> {

        let mut results: Vec<i32> = Vec::new();
        for i in &self.cells {
            if i.state == 'U' {
                results.push(1)
            } else {
                results.push(-1)
            }
        }
        results
    }

}


// fn main() {

//     let mut grid = Grid::new();
//     for i in 1..=10000 {
//         let float_i = i as f64;
//         grid.set_temperature(1001.0 - float_i/10.);
//         let energy = grid.advance();
//         // if i%100 == 0 {
//         //     println!("{}",energy);
//         // }
//     }
// }
