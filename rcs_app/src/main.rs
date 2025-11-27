use eframe::egui;
use rcs_core::{Updatable, World};

struct SimulationApp {
    world: World,
    paused: bool,
    cell_px: f32,
    tick: u64,

    world_width: usize,
    world_height: usize,
    max_agents: usize,
}

impl SimulationApp {
    fn new() -> Self {
        let world = World::make_simple_world();
        let (w, h) = world.size();
        let max_agents = world.max_agents();

        Self {
            world,
            paused: false,
            cell_px: 25.0,
            tick: 0,
            world_width: w,
            world_height: h,
            max_agents,
        }
    }
}

impl eframe::App for SimulationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.world =
                        World::make_world(self.world_width, self.world_height, self.max_agents);
                    self.tick = 0;
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

                ui.label("Cell size:");
                ui.add(egui::Slider::new(&mut self.cell_px, 5.0..=100.0).text("px"));

                ui.horizontal(|ui| {
                    ui.label("World W x H:");
                    ui.add(egui::DragValue::new(&mut self.world_width).range(5..=200));
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut self.world_height).range(5..=200));
                    ui.label("Max agents:");
                    ui.add(egui::DragValue::new(&mut self.max_agents).range(1..=2000));
                });

                ui.label(format!("Tick: {}", self.tick));
            });
        });

        if !self.paused {
            let _ = self.world.update();
            self.tick += 1;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            draw_world(ui, &self.world, self.cell_px);
        });
        ctx.request_repaint();
    }
}

fn draw_world(ui: &mut egui::Ui, world: &World, cell_px: f32) {
    let (width, height) = world.size();
    let world_width_px = width as f32 * cell_px;
    let world_height_px = height as f32 * cell_px;
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(world_width_px, world_height_px),
        egui::Sense::hover(),
    );
    let painter = ui.painter_at(rect);

    for y in 0..height {
        for x in 0..width {
            let cid = y * width + x;
            let cell = world.cell(cid);
            let resource = cell.cur_resource() as f32;

            let min = rect.min + egui::vec2(x as f32 * cell_px, y as f32 * cell_px);
            let max = min + egui::vec2(cell_px, cell_px);
            let cell_rect = egui::Rect::from_min_max(min, max);

            let t = (resource / 20.0).clamp(0.0, 1.0);
            let color = egui::Color32::from_rgb(
                (30.0 + t * 80.0) as u8,
                (80.0 + t * 140.0) as u8,
                (120.0 - t * 60.0) as u8,
            );

            painter.rect_filled(cell_rect, 0.0, color);
            painter.rect_stroke(
                cell_rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                egui::StrokeKind::Inside,
            );
        }
    }

    for agent in world.agents() {
        if !agent.is_alive() {
            continue;
        }

        let cid = agent.cid();
        let ax = cid % width;
        let ay = cid / width;

        let center =
            rect.min + egui::vec2((ax as f32 + 0.5) * cell_px, (ay as f32 + 0.5) * cell_px);
        let max_hp = 3.0;
        let hp = agent.health_point() as f32;
        let t = (hp / max_hp).clamp(0.0, 1.0);
        let color = egui::Color32::from_rgb(255, (255.0 * t) as u8, (255.0 * t) as u8);
        painter.circle_filled(center, cell_px * 0.35, color);
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "2D Resource Competition",
        options,
        Box::new(|_cc| Ok(Box::new(SimulationApp::new()))),
    );
}
