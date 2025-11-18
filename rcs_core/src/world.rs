use crate::errors::SimulationError;
use crate::traits::Updatable;
use crate::{Agent, Cell};

pub struct World {
    size: (usize, usize),
    cells: Vec<Cell>,
    agents: Vec<Agent>,
    max_agents: usize,
}

impl World {
    pub fn new(
        size: (usize, usize),
        cells: Vec<Cell>,
        agents: Vec<Agent>,
        max_agents: usize,
    ) -> Self {
        Self {
            size,
            cells,
            agents,
            max_agents,
        }
    }
}

impl Updatable for World {
    fn update(&mut self) -> Result<(), SimulationError> {
        Ok(())
    }
}
