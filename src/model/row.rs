#[derive(Clone)]
pub struct Row {
    pub cells: Vec<String>,
    pub is_disabled: bool,
}

impl Row {
    pub fn new(length: usize, is_disabled: bool) -> Self {
        Self {
            cells: vec!["".into(); length],
            is_disabled,
        }
    }
}
