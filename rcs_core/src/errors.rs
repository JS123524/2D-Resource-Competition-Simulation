#[derive(Debug)]
pub enum SimulationError {
    NotAlive,
    NotEnoughResources { available: u32 },
}
