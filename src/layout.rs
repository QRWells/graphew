use egui_graphs::Graph;
use petgraph::Directed;

use self::radial::RadialLayout;

pub mod radial;

#[derive(Debug, PartialEq)]
pub enum Layout {
    Radial,
}

impl Layout {
    pub fn layout<N: Clone, E: Clone>(&self, graph: &mut Graph<N, E, Directed>) {
        match self {
            Layout::Radial => RadialLayout::layout(graph),
        }
    }
}
