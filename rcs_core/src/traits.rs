use crate::errors::SimulationError;

pub trait Updatable {
    fn update(&mut self) -> Result<(), SimulationError>;
}
