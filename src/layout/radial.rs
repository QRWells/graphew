use std::{
    collections::{HashSet, VecDeque},
    f32::consts::PI,
};

#[derive(Debug, Default)]
pub struct RadialLayout;

impl RadialLayout {
    pub fn layout<N: Clone, E: Clone>(graph: &mut egui_graphs::Graph<N, E, petgraph::Directed>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let g_indices = graph.node_indices().collect::<Vec<_>>();
        let root = g_indices[0];
        visited.insert(root.index());
        let g_node = graph.node_weight_mut(root).unwrap();
        g_node.set_location(egui::Vec2 { x: 0.0, y: 0.0 });
        let childs = graph
            .neighbors_directed(root, petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();
        let len = childs.len() as f32;
        for (i, child) in childs.iter().enumerate() {
            let node = graph.node_weight_mut(*child).unwrap();
            let relative_rad = (i + 1) as f32 * 2.0 * PI / len;
            let location =
                polar_to_cartesian(relative_rad, 160.0, egui::Vec2 { x: 0.0, y: 0.0 }, 0.0);
            node.set_location(location);
            queue.push_back((*child, len > 1.0, relative_rad + PI, 160.0));
        }

        while let Some((node_idx, sibling, rad, parent)) = queue.pop_front() {
            if visited.contains(&node_idx.index()) {
                continue;
            }
            visited.insert(node_idx.index());
            let node = graph.node_weight_mut(node_idx).unwrap();
            let r = calc_r(parent, sibling);
            let origin = node.location();

            let childs = graph
                .neighbors_directed(node_idx, petgraph::Direction::Outgoing)
                .filter(|child| !visited.contains(&child.index()))
                .collect::<Vec<_>>();
            let len = childs.len() as f32;
            for (i, child) in childs.iter().enumerate() {
                let node = graph.node_weight_mut(*child).unwrap();
                let relative_rad = calc_non_root_theta(PI, 1 + i, len);
                let location = polar_to_cartesian(relative_rad, r, origin, rad);
                node.set_location(location);
                queue.push_back((*child, len > 1.0, relative_rad + rad + PI, r));
            }
        }
    }
}

fn polar_to_cartesian(theta: f32, r: f32, origin: egui::Vec2, rad: f32) -> egui::Vec2 {
    egui::Vec2 {
        x: r * (theta + rad).cos(),
        y: r * (theta + rad).sin(),
    } + origin
}

fn calc_non_root_theta(phi: f32, i: usize, m: f32) -> f32 {
    PI - (phi / 2.0) + ((i as f32) / m * phi) + (phi / (2.0 * m))
}

fn calc_r(v: f32, sibling: bool) -> f32 {
    match sibling {
        true => v / 2.0,
        false => v / 2.0,
    }
}
