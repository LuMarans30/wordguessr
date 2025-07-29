#[derive(Clone)]
pub struct Dialog {
    id: String,
    pub title: String,
    pub message: String,
    pub is_active: bool,
}

impl Dialog {
    pub fn new(id: String, title: String, message: String, is_active: bool) -> Self {
        Self {
            id,
            title,
            message,
            is_active,
        }
    }

    pub fn warning_dialog(message: String, is_active: bool) -> Self {
        Self::new(
            "warning-dialog".into(),
            "Warning".into(),
            message,
            is_active,
        )
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
