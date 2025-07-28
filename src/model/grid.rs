use crate::model::row::Row;

pub struct Grid {
    pub rows: Vec<Row>,
    word_length: usize,
    pub current_row: usize,
}

impl Grid {
    pub fn new(tries: usize, word_length: usize) -> Self {
        let current_row = 0;
        let rows = (0..tries)
            .map(|i| Row::new(word_length, i != current_row))
            .collect();
        Self {
            rows,
            word_length,
            current_row,
        }
    }

    pub fn set_next_row(&mut self) {
        self.rows[self.current_row].is_disabled = true;
        if self.current_row < self.rows.len() - 1 {
            self.current_row += 1;
            self.rows[self.current_row].is_disabled = false;
        }
    }

    pub fn reset(&mut self) {
        self.current_row = 0;
        for row in &mut self.rows {
            *row = Row::new(self.word_length, true);
        }
        self.rows[self.current_row].is_disabled = false;
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(5, 5)
    }
}
