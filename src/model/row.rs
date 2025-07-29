use crate::model::cell::Cell;

#[derive(Clone)]
pub struct Row {
    pub cells: Vec<Cell>,
    is_disabled: bool,
}

impl Row {
    pub fn new(length: usize, is_disabled: bool) -> Self {
        Self {
            cells: vec![Cell::new(None, is_disabled); length],
            is_disabled,
        }
    }

    pub fn set_disabled(&mut self, is_disabled: bool) {
        self.is_disabled = is_disabled;
        self.cells
            .iter_mut()
            .for_each(|cell| cell.is_disabled = is_disabled);
    }

    pub fn is_disabled(&self) -> bool {
        self.is_disabled
    }
}
