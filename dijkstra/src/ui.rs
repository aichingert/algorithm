use eframe::egui;

use crate::dijkstra::Dijkstra;
use crate::node::Node;

pub fn app() {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Dijkstra visualizer",
        options,
        Box::new(|_cc| Box::new(Dijkstra::default())),
    );
}

impl eframe::App for Dijkstra {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dijkstra visualizer");
            ui.horizontal(|ui| {
                ui.label("Node value: ");
                ui.text_edit_singleline(&mut self.current_node_value);
            });
            if ui.button("Add node").clicked() {
                if !self.current_node_value.parse::<i32>().is_err() {
                    let node: Node = Node::new(self.current_node_value.parse::<i32>().unwrap(), vec![]);
                    self.nodes.push(node);
                    println!("{:?}", self);
                } else {
                    ui.label("Please enter a valid value for the node");
                }
            }
        });
    }
}