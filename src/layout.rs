use egui_graphs::Graph;
use petgraph::Directed;

use self::radical::RadicalLayout;

pub mod radical;

#[derive(Debug, PartialEq)]
pub enum Layout {
    Radical,
}

impl Layout {
    pub fn layout<N: Clone, E: Clone>(&self, graph: &mut Graph<N, E, Directed>) {
        match self {
            Layout::Radical => RadicalLayout::layout(graph),
        }
    }
}
