mod app;
mod world_view;
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "2D Resource Competition",
        options,
        Box::new(|_cc| Ok(Box::new(app::SimulationApp::new()))),
    );
}
