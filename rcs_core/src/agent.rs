use crate::errors::SimulationError;
use crate::traits::Updatable;

pub struct Agent {
    id: usize,
    cid: usize,
    consumption_rate: u32,
    allocated_resource: u32,
    health_point: u32,
    alive: bool,
}

impl Agent {
    pub fn new(
        id: usize,
        cid: usize,
        consumption_rate: u32,
        allocated_resource: u32,
        health_point: u32,
        alive: bool,
    ) -> Self {
        Self {
            id,
            cid,
            consumption_rate,
            allocated_resource,
            health_point,
            alive,
        }
    }

    fn metabolize(&mut self) {
        if self.allocated_resource < self.consumption_rate {
            self.health_point = self.health_point.saturating_sub(1);
        }
        self.allocated_resource = 0;
        if self.health_point == 0 {
            self.alive = false;
        }
    }

    fn movement_cost(&mut self) {
        self.health_point = self.health_point.saturating_sub(1);
        if self.health_point == 0 {
            self.alive = false;
        }
    }

    pub fn move_to(&mut self, new_id: usize) -> Result<(), SimulationError> {
        if !self.alive {
            return Err(SimulationError::NotAlive);
        }
        self.cid = new_id;
        self.movement_cost();
        Ok(())
    }

    pub fn retrieve_resource(&mut self, resource: u32) -> u32 {
        let take = resource.min(self.consumption_rate);
        self.allocated_resource = take;
        resource - take
    }

    pub fn decide_move(&self, neighbor_cells: &[(usize, u32)]) -> Option<usize> {
        if neighbor_cells.is_empty() {
            return None;
        }
        let best = neighbor_cells
            .iter()
            .copied()
            .max_by_key(|&(_cid, resource)| resource);
        match best {
            Some((cid, resource)) if resource > 0 => Some(cid),
            _ => None,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn cid(&self) -> usize {
        self.cid
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn is_hungry(&self) -> bool {
        self.allocated_resource < self.consumption_rate
    }

    pub fn health_point(&self) -> u32 {
        self.health_point
    }
}

impl Updatable for Agent {
    fn update(&mut self) -> Result<(), SimulationError> {
        if !self.alive {
            return Err(SimulationError::NotAlive);
        }
        self.metabolize();
        Ok(())
    }
}
