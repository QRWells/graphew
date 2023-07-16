#[derive(Debug, Clone)]
pub struct State {
    pub index: usize,
    pub info: i32,
}

impl State {
    pub fn new(index: usize, info: i32) -> Self {
        Self { index, info }
    }
}