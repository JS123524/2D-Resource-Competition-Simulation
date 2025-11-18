use crate::errors::SimulationError;
use crate::traits::Updatable;

pub struct Cell {
    id: usize,
    position: (u32, u32),
    size: (u32, u32),
    cur_resource: u32,
    max_resource: u32,
    regen_rate: u32,
    max_regen_rate: u32,
}

impl Cell {
    pub fn new(
        id: usize,
        position: (u32, u32),
        size: (u32, u32),
        cur_resource: u32,
        max_resource: u32,
        regen_rate: u32,
        max_regen_rate: u32,
    ) -> Self {
        Self {
            id,
            position,
            size,
            cur_resource,
            max_resource,
            regen_rate,
            max_regen_rate,
        }
    }

    pub fn add_resource(&mut self, resource: u32) {
        self.cur_resource = (self.cur_resource.saturating_add(resource)).min(self.max_resource);
    }

    pub fn resource_consumption(&mut self, resource: u32) -> Result<u32, SimulationError> {
        if self.cur_resource < resource {
            return Err(SimulationError::NotEnoughResources {
                available: self.cur_resource,
            });
        }
        self.cur_resource -= resource;
        Ok(resource)
    }

    pub fn take_up_to(&mut self, want: u32) -> u32 {
        let take = want.min(self.cur_resource);
        self.cur_resource -= take;
        take
    }

    pub fn increase_rate(&mut self, regen_rate: u32) {
        self.regen_rate = (self.regen_rate.saturating_add(regen_rate)).min(self.max_regen_rate);
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Updatable for Cell {
    fn update(&mut self) -> Result<(), SimulationError> {
        self.cur_resource =
            (self.cur_resource.saturating_add(self.regen_rate)).min(self.max_resource);
        Ok(())
    }
}
