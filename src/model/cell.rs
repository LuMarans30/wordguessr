use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum CellState {
    Empty,
    Correct,
    Present,
    Absent,
}

#[derive(Clone)]
pub struct Cell {
    pub letter: Option<char>,
    pub state: CellState,
    pub is_disabled: bool,
}

impl Cell {
    pub fn new(letter: Option<char>, is_disabled: bool) -> Self {
        Self {
            letter,
            is_disabled,
            state: CellState::Empty,
        }
    }

    pub fn with_state(mut self, state: CellState) -> Self {
        self.state = state;
        self
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self.letter {
            Some(letter) => letter.into(),
            None => String::new(),
        };
        write!(f, "{letter}")
    }
}
