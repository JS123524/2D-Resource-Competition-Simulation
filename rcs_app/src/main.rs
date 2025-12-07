//! GUI entry point for the 2-D resource competition simulation.
//!
//! This binary sets up an `eframe` window and launches `SimulationApp`,
//! which renders the world and drives the simulation loop.
mod app;
mod world_view;

/// Starts the native `eframe` application.
///
/// This creates a window titled `"2D Resource Competition"` and
/// runs [`app::SimulationApp`] as the main UI state.
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "2D Resource Competition",
        options,
        Box::new(|_cc| Ok(Box::new(app::SimulationApp::new()))),
    );
}
