#[derive(Debug, Clone)]
pub struct Transition {
    pub from: usize,
    pub to: usize,
}

impl Transition {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}
