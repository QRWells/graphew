#[derive(Debug, Clone)]
pub struct State {
    pub index: usize,
    pub info: String,
}

impl State {
    pub fn new(index: usize, info: String) -> Self {
        Self { index, info }
    }
}
