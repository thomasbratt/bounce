use crate::model::ShapeIndex;
use crate::shape::Shape;

#[derive(Debug)]
pub struct Occupy {
    cells: Vec<Vec<ShapeIndex>>,
    cell_width: u32,
    cell_height: u32,
    cells_in_row: u32,
}

impl Occupy {
    // TODO: return error here
    pub fn new(cell_width: u32, cell_height: u32, world_width: u32, world_height: u32) -> Self {
        let cells_in_row = world_width / cell_width;
        let cells_in_column = world_height / cell_height;
        let mut cells = Vec::with_capacity((cells_in_row * cells_in_column) as usize);
        Occupy {
            cells,
            cell_width,
            cell_height,
            cells_in_row,
        }
    }

    pub fn initialize(mut self, shapes: &Vec<Shape>) -> Self {
        for (i, s) in shapes.iter().enumerate() {
            for o in self.determine_occupancy(s) {
                self.cells.get_mut(o).unwrap().push(i as ShapeIndex);
            }
        }
        self
    }

    // TODO: unit tests
    fn determine_occupancy(&self, shape: &Shape) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for x in (shape.x) / self.cell_width..=(shape.x + shape.width) / self.cell_width {
            for y in (shape.y) / self.cell_height..=(shape.y + shape.width) / self.cell_height {
                result.push((x + y * self.cells_in_row) as usize);
            }
        }
        result
    }
}
