/// Errors that can occur during the simulation.
///
/// This enum is used by components such as `Agent` and `Cell`
/// to report exceptional situations, instead of panicking.
#[derive(Debug)]
pub enum SimulationError {
    NotAlive,
    NotEnoughResources { available: u32 },
}
