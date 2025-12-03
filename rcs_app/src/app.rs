use std::time::Instant;

use eframe::egui;
use rcs_core::{Updatable, World, WorldConfig};

use crate::world_view;
pub struct SimulationApp {
    world: World,
    config: WorldConfig,

    paused: bool,
    cell_px: f32,
    tick: u64,

    step_interval: f32,
    last_step: Instant,
}

impl SimulationApp {
    pub fn new() -> Self {
        let config = WorldConfig::default();
        let world = World::from_config(config);

        Self {
            world,
            config,
            paused: false,
            cell_px: 25.0,
            tick: 0,
            step_interval: 0.2,
            last_step: Instant::now(),
        }
    }
}

impl eframe::App for SimulationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.world = World::from_config(self.config);
                    self.tick = 0;
                    self.last_step = Instant::now();
                }

                if ui
                    .button(if self.paused {
                        "▶ Resume"
                    } else {
                        "⏸ Pause"
                    })
                    .clicked()
                {
                    self.paused = !self.paused;
                }

                if ui.button("Step").clicked() {
                    let _ = self.world.update();
                    self.tick += 1;
                }

                ui.separator();
                ui.label(format!("Tick: {}", self.tick));
            });
        });

        egui::SidePanel::right("side_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("View");
                ui.label("Cell size:");
                ui.add(egui::Slider::new(&mut self.cell_px, 5.0..=100.0).text("px"));
                ui.separator();

                ui.heading("Simulation Speed");
                ui.label("Seconds per tick:");
                ui.add(egui::Slider::new(&mut self.step_interval, 0.01..=1.0).text("s"));
                ui.separator();

                world_config_ui(ui, &mut self.config);
            });

        if !self.paused {
            let now = Instant::now();
            let dt = now.duration_since(self.last_step).as_secs_f32();

            if dt >= self.step_interval {
                let _ = self.world.update();
                self.tick += 1;
                self.last_step = now;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            world_view::draw_world(ui, &self.world, &self.config, self.cell_px);
        });
        ctx.request_repaint();
    }
}

fn world_config_ui(ui: &mut egui::Ui, cfg: &mut WorldConfig) {
    ui.heading("World Config");
    ui.label("World W x H:");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut cfg.width).range(3..=200));
        ui.label("x");
        ui.add(egui::DragValue::new(&mut cfg.height).range(3..=200));
    });
    ui.separator();

    ui.heading("Cell / Agent Init Ranges");
    egui::Grid::new("world_config_grid")
        .num_columns(4)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("");
            ui.label("min");
            ui.label("max");
            ui.end_row();

            ui.label("Cell: initial resource");
            ui.add(egui::DragValue::new(&mut cfg.min_resource).range(0..=cfg.max_resource));
            ui.add(egui::DragValue::new(&mut cfg.max_resource).range(0..=100));
            ui.end_row();

            ui.label("Cell: regen per tick");
            ui.add(egui::DragValue::new(&mut cfg.min_regen_rate).range(0..=cfg.max_regen_rate));
            ui.add(egui::DragValue::new(&mut cfg.max_regen_rate).range(0..=10));
            ui.end_row();

            ui.label("Agents: initial count");
            ui.add(egui::DragValue::new(&mut cfg.min_agents).range(1..=cfg.max_agents));
            ui.add(egui::DragValue::new(&mut cfg.max_agents).range(1..=2000));
            ui.end_row();

            ui.label("Agent: consumption per tick");
            ui.add(
                egui::DragValue::new(&mut cfg.min_consumption_rate)
                    .range(1..=cfg.max_consumption_rate),
            );
            ui.add(egui::DragValue::new(&mut cfg.max_consumption_rate).range(1..=10));
            ui.end_row();
        });
    ui.label("All ranges above are sampled uniformly from [min, max].");
    ui.separator();

    ui.label("Agent HP (initial, fixed):");
    ui.add(egui::DragValue::new(&mut cfg.agent_hp).range(1..=30));
    ui.separator();
}
