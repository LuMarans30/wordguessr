use crate::model::row::Row;

#[derive(Clone, Debug)]
pub struct Grid {
    pub rows: Vec<Row>,
    pub current_row: usize,
    num_rows: usize,
}

impl Grid {
    pub fn new(num_rows: usize, word_length: usize) -> Self {
        let rows = (0..num_rows)
            .map(|i| Row::new(word_length, i != 0))
            .collect();

        Self {
            rows,
            current_row: 0,
            num_rows,
        }
    }

    pub fn can_advance(&self) -> bool {
        self.current_row < self.num_rows - 1
    }

    pub fn advance_row(&mut self) -> Result<(), GridError> {
        self.rows[self.current_row].set_disabled(true);
        if !self.can_advance() {
            return Err(GridError::NoMoreRows);
        }

        self.current_row += 1;
        self.rows[self.current_row].set_disabled(false);
        Ok(())
    }
}

#[derive(Debug)]
pub enum GridError {
    NoMoreRows,
}
