use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Auth {
    pub is_ok: usize,
}
