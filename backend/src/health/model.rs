use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Health {
    pub is_ok: usize,
}

pub struct Test {
    pub test: usize,
}
