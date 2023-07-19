use std::path::PathBuf;

use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{FontId, Layout, RichText, ScrollArea, Vec2, Visuals};
use egui_graphs::{
    Change, Graph, GraphView, Node, SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use fdg_sim::{glam::Vec3, Simulation};
use petgraph::Directed;

use crate::{
    graph::{
        construct_simulation,
        state::State,
        transition::Transition,
        translator::{SLIMTranslator, Translator},
    },
    layout,
    settings::{self},
    views::about::AboutWindow,
};

const SIMULATION_DT: f32 = 0.015;

pub struct MainApp {
    file: Option<PathBuf>,

    graph: Graph<State, Transition, Directed>,
    sim: Simulation<State, f32>,
    loaded: bool,
    layout: layout::Layout,

    selected_nodes: Vec<Node<State>>,

    about: Option<AboutWindow>,
    settings_interaction: settings::SettingsInteraction,
    settings_navigation: settings::SettingsNavigation,
    settings_style: settings::SettingsStyle,

    changes_receiver: Receiver<Change>,
    changes_sender: Sender<Change>,

    dark_mode: bool,
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (changes_sender, changes_receiver) = unbounded();
        Self {
            file: None,
            graph: Graph::new(),
            sim: construct_simulation(&Graph::new()),
            loaded: false,
            layout: layout::Layout::Radical,
            selected_nodes: vec![],
            about: None,
            settings_interaction: settings::SettingsInteraction::default(),
            settings_navigation: settings::SettingsNavigation::default(),
            settings_style: settings::SettingsStyle::default(),
            changes_receiver,
            changes_sender,
            dark_mode: false,
        }
    }

    pub fn handle_changes(&self) {
        self.changes_receiver
            .try_iter()
            .for_each(|change| match change {
                Change::Node(_) => {}
                Change::Edge(_) => {}
                Change::SubGraph(_) => {}
            });
    }

    fn update_simulation(&mut self) {
        let looped_nodes = {
            let graph = self.sim.get_graph_mut();
            let mut looped_nodes = vec![];
            let mut looped_edges = vec![];
            graph.edge_indices().for_each(|idx| {
                let edge = graph.edge_endpoints(idx).unwrap();
                let looped = edge.0 == edge.1;
                if looped {
                    looped_nodes.push((edge.0, ()));
                    looped_edges.push(idx);
                }
            });

            for idx in looped_edges {
                graph.remove_edge(idx);
            }

            self.sim.update(SIMULATION_DT);

            looped_nodes
        };

        let graph = self.sim.get_graph_mut();
        for (idx, _) in looped_nodes.iter() {
            graph.add_edge(*idx, *idx, 1.);
        }
    }

    fn sync_graph_with_simulation(&mut self) {
        self.selected_nodes = vec![];

        let g_indices = self.graph.node_indices().collect::<Vec<_>>();
        g_indices.iter().for_each(|g_n_idx| {
            let g_n = self.graph.node_weight_mut(*g_n_idx).unwrap();
            let sim_n = self.sim.get_graph_mut().node_weight_mut(*g_n_idx).unwrap();

            if g_n.dragged() {
                let loc = g_n.location();
                sim_n.location = Vec3::new(loc.x, loc.y, 0.);
                return;
            }

            let loc = sim_n.location;
            g_n.set_location(Vec2::new(loc.x, loc.y));

            if g_n.selected() {
                self.selected_nodes.push(g_n.clone());
            }
        });

        // reset the weights of the edges
        self.sim.get_graph_mut().edge_weights_mut().for_each(|w| {
            *w = 1.;
        });
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(self.loaded, |ui| {
            egui::Grid::new("slider settings")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("selection depth");
                    ui.add(egui::Slider::new(
                        &mut self.settings_interaction.selection_depth,
                        -10..=10,
                    ));
                    ui.end_row();
                });

            ui.separator();

            ui.checkbox(&mut self.settings_style.labels_always, "show labels");

            ui.separator();

            egui::ComboBox::from_label("Layout").show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut self.layout, layout::Layout::Radical, "Radical");
            });

            if ui.button("Layout").on_hover_text("Layout the graph").clicked() {
                self.layout.layout(&mut self.graph);
            }
        });
    }
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.dark_mode {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        if let Some(about) = self.about.as_mut() {
            let mut is_open = true;
            about.show(ctx, &mut is_open);

            if !is_open {
                self.about = None;
            }
        }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Kripke").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Kripke json", &["json"])
                            .pick_file()
                        {
                            self.file = Some(path);
                        }
                    }
                    if ui.button("Open slim States").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("slim dump", &["*"])
                            .pick_file()
                        {
                            let file = std::fs::read_to_string(path).unwrap();
                            if let Ok(state_space) = SLIMTranslator::translate(&file) {
                                self.graph = state_space.into();
                                self.sim = construct_simulation(&self.graph);
                                self.loaded = true;
                            }
                        }
                    }
                    if ui.button("Open SPIN States").clicked() {
                        // todo
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui
                        .button("About")
                        .on_hover_text("Show about dialog")
                        .clicked()
                    {
                        self.about = Some(AboutWindow);
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // todo: file type
                egui::warn_if_debug_build(ui);
            });
        });

        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Control Panel");

            self.settings(ui);

            ui.collapsing("selected", |ui| {
                ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                    self.selected_nodes.iter().for_each(|node| {
                        if let Some(state) = node.data() {
                            ui.label(
                                RichText::new(format!("{}", state.info))
                                    .font(FontId::proportional(18.0)),
                            );
                        }
                    });
                });
            });

            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                if ui
                    .button({
                        match self.dark_mode {
                            true => "🔆 light",
                            false => "🌙 dark",
                        }
                    })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.loaded {
                let interaction_settings = SettingsInteraction::new()
                    .with_dragging_enabled(true)
                    .with_clicking_enabled(true)
                    .with_selection_enabled(true)
                    .with_selection_depth(self.settings_interaction.selection_depth)
                    .with_folding_enabled(self.settings_interaction.folding_enabled)
                    .with_folding_depth(self.settings_interaction.folding_depth);

                let navi_settings = SettingsNavigation::new()
                    .with_fit_to_screen_enabled(false)
                    .with_zoom_and_pan_enabled(self.settings_navigation.zoom_and_pan_enabled)
                    .with_screen_padding(self.settings_navigation.screen_padding)
                    .with_zoom_speed(self.settings_navigation.zoom_speed);

                let style_settings = SettingsStyle::new()
                    .with_edge_radius_weight(self.settings_style.edge_radius_weight)
                    .with_folded_radius_weight(self.settings_style.folded_node_radius_weight)
                    .with_labels_always(self.settings_style.labels_always);

                let mut graph = GraphView::new(&mut self.graph)
                    .with_interactions(&interaction_settings)
                    .with_navigations(&navi_settings)
                    .with_styles(&style_settings)
                    .with_changes(&self.changes_sender);

                ui.add(&mut graph);
            }
        });

        self.handle_changes();
        self.sync_graph_with_simulation();
        self.update_simulation();
    }
}
