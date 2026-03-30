#[derive(Debug, Clone)]
pub struct DbOptions {
    pub create_if_missing: bool,
    pub sync_write: bool,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            create_if_missing: true,
            sync_write: false,
        }
    }
}
