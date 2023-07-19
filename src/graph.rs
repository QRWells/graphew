use std::collections::HashMap;

use egui_graphs::{to_input_graph, Graph};
use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
use petgraph::{stable_graph::StableGraph, visit::IntoNodeReferences, Directed};

pub mod state;
pub mod transition;
pub mod translator;

#[derive(Debug, Clone)]
pub struct StateSpace {
    pub states: Vec<state::State>,
    pub transitions: Vec<transition::Transition>,
}

impl Into<Graph<state::State, transition::Transition, Directed>> for StateSpace {
    fn into(self) -> Graph<state::State, transition::Transition, Directed> {
        let mut g = StableGraph::new();
        let mut idx_map = HashMap::new();

        for state in &self.states {
            let idx = g.add_node(state.clone());
            idx_map.insert(state.index, idx);
        }

        for transition in &self.transitions {
            g.add_edge(
                idx_map.get(&transition.from).unwrap().clone(),
                idx_map.get(&transition.to).unwrap().clone(),
                transition.clone(),
            );
        }

        to_input_graph(&g)
    }
}

pub fn construct_simulation(
    g: &Graph<state::State, transition::Transition, Directed>,
) -> Simulation<state::State, f32> {
    // create force graph
    let mut force_graph = ForceGraph::with_capacity(g.node_count(), g.edge_count());
    g.node_references().for_each(|(idx, node)| {
        if let Some(state) = node.data() {
            force_graph.add_force_node(format!("{}", idx.index()).as_str(), state.clone());
        }
    });
    g.edge_indices().for_each(|idx| {
        let (source, target) = g.edge_endpoints(idx).unwrap();
        force_graph.add_edge(source, target, 1.);
    });

    // initialize simulation
    let mut params = SimulationParameters::default();
    let force = fdg_sim::force::fruchterman_reingold_weighted(100., 0.95);
    params.set_force(force);

    Simulation::from_graph(force_graph, params)
}
