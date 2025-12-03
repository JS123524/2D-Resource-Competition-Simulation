use eframe::egui;
use rcs_core::{World, WorldConfig};

pub fn draw_world(ui: &mut egui::Ui, world: &World, cfg: &WorldConfig, cell_px: f32) {
    let (width, height) = world.size();
    let world_width_px = width as f32 * cell_px;
    let world_height_px = height as f32 * cell_px;
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(world_width_px, world_height_px),
        egui::Sense::hover(),
    );
    let painter = ui.painter_at(rect);

    let max_res_f = cfg.max_resource.max(1) as f32;
    let max_hp_f = cfg.agent_hp.max(1) as f32;

    for y in 0..height {
        for x in 0..width {
            let cid = y * width + x;
            let cell = world.cell(cid);
            let resource = cell.cur_resource() as f32;

            let min = rect.min + egui::vec2(x as f32 * cell_px, y as f32 * cell_px);
            let max = min + egui::vec2(cell_px, cell_px);
            let cell_rect = egui::Rect::from_min_max(min, max);

            let t = (resource / max_res_f).clamp(0.0, 1.0);
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
        let hp = agent.health_point() as f32;
        let t = (hp / max_hp_f).clamp(0.0, 1.0);
        let color = egui::Color32::from_rgb(255, (255.0 * t) as u8, (255.0 * t) as u8);
        painter.circle_filled(center, cell_px * 0.35, color);
    }
}
