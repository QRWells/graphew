use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{Layout, Visuals};
use egui_graphs::{
    to_input_graph, Change, Graph, GraphView, SettingsInteraction, SettingsNavigation,
    SettingsStyle,
};
use petgraph::{stable_graph::StableGraph, Directed};

use crate::{
    graph::{state::State, transition::Transition},
    settings::{self},
    views::about::AboutWindow,
};

pub struct MainApp {
    graph: Graph<State, Transition, Directed>,
    about: Option<AboutWindow>,
    settings_interaction: settings::SettingsInteraction,
    settings_navigation: settings::SettingsNavigation,
    settings_style: settings::SettingsStyle,

    changes_receiver: Receiver<Change>,
    changes_sender: Sender<Change>,

    dark_mode: bool,
}

fn generate_graph() -> Graph<State, Transition, Directed> {
    let mut g = StableGraph::new();

    let a = g.add_node(State { index: 0, info: 0 });
    let b = g.add_node(State { index: 1, info: 1 });
    let c = g.add_node(State { index: 2, info: 2 });
    let d = g.add_node(State { index: 3, info: 3 });
    let e = g.add_node(State { index: 4, info: 4 });
    let f = g.add_node(State { index: 5, info: 5 });

    g.add_edge(a, b, Transition { from: 0, to: 1 });
    g.add_edge(b, c, Transition { from: 1, to: 2 });
    g.add_edge(c, a, Transition { from: 2, to: 0 });
    g.add_edge(c, d, Transition { from: 2, to: 3 });
    g.add_edge(d, e, Transition { from: 3, to: 4 });
    g.add_edge(e, f, Transition { from: 4, to: 5 });
    g.add_edge(f, d, Transition { from: 5, to: 3 });

    to_input_graph(&g)
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (changes_sender, changes_receiver) = unbounded();
        Self {
            graph: generate_graph(),
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
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { graph, about, .. } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.dark_mode {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        if let Some(about) = about.as_mut() {
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
                    if ui.button("New").clicked() {}
                    if ui.button("Open").clicked() {}
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
                ui.label("Ready");
                egui::warn_if_debug_build(ui);
            });
        });

        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Control Panel");

            ui.add(
                egui::Slider::new(&mut self.settings_interaction.selection_depth, -10..=10)
                    .text("depth"),
            );

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

            let mut graph = GraphView::new(graph)
                .with_interactions(&interaction_settings)
                .with_navigations(&navi_settings)
                .with_styles(&style_settings)
                .with_changes(&self.changes_sender);

            ui.add(&mut graph);
        });

        self.handle_changes();
    }
}
