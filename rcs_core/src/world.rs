use crate::errors::SimulationError;
use crate::traits::Updatable;
use crate::{Agent, Cell};
use rand::Rng;

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

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn cell(&self, cid: usize) -> &Cell {
        &self.cells[cid]
    }

    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    pub fn max_agents(&self) -> usize {
        self.max_agents
    }

    pub fn make_world(width: usize, height: usize, max_agents: usize) -> Self {
        assert!(width > 0 && height > 0, "world size must be > 0");
        assert!(max_agents > 0, "max_agents must be > 0");

        let mut cells = Vec::with_capacity(width * height);
        let max_resource: u32 = 20;
        let max_regen_rate: u32 = 3;

        let mut rng = rand::thread_rng();

        for y in 0..height {
            for x in 0..width {
                let id = y * width + x;
                let rand_resource = rng.gen_range(0..=max_resource);
                let rand_regen_rate = rng.gen_range(0..=max_regen_rate);

                cells.push(Cell::new(
                    id,
                    rand_resource,
                    max_resource,
                    rand_regen_rate,
                    max_regen_rate,
                ));
            }
        }

        let num_agents = rng.gen_range(1..=max_agents);
        let mut agents = Vec::with_capacity(num_agents);
        let max_consumption_rate: u32 = 5;

        for id in 0..num_agents {
            let rand_x = rng.gen_range(0..width);
            let rand_y = rng.gen_range(0..height);
            let cid = rand_y * width + rand_x;
            let rand_consumption_rate = rng.gen_range(1..=max_consumption_rate);

            agents.push(Agent::new(id, cid, rand_consumption_rate, 0, 3, true));
        }

        World::new((width, height), cells, agents, max_agents)
    }

    pub fn make_simple_world() -> Self {
        /*
        let (width, height) = (20, 20);
        let mut cells = Vec::with_capacity(width * height);

        let max_resource: u32 = 20;
        let max_regen_rate: u32 = 3;

        let mut rng = rand::thread_rng();

        for y in 0..height {
            for x in 0..width {
                let id = y * width + x;
                let rand_resource = rng.gen_range(0..=max_resource);
                let rand_regen_rate = rng.gen_range(0..=max_regen_rate);

                cells.push(Cell::new(
                    id,
                    rand_resource,
                    max_resource,
                    rand_regen_rate,
                    max_regen_rate,
                ));
            }
        }

        let max_agents: usize = 50;
        let num_agents = rng.gen_range(1..=max_agents);
        let mut agents = Vec::with_capacity(num_agents);
        let max_consumption_rate: u32 = 5;

        for id in 0..num_agents {
            let rand_x = rng.gen_range(0..width);
            let rand_y = rng.gen_range(0..height);
            let cid = rand_y * width + rand_x;
            let rand_consumption_rate = rng.gen_range(1..=max_consumption_rate);

            agents.push(Agent::new(id, cid, rand_consumption_rate, 0, 3, true));
        }

        World::new((width, height), cells, agents, max_agents)
        */
        Self::make_world(20, 20, 50)
    }

    fn neighbor_cells_info(&self, cid: usize) -> Vec<(usize, u32)> {
        let (width, height) = self.size;
        let x = cid % width;
        let y = cid / width;

        let mut neighbors = Vec::with_capacity(4);

        if y > 0 {
            let ny = y - 1;
            let nid = ny * width + x;
            neighbors.push((nid, self.cells[nid].cur_resource()));
        }

        if y + 1 < height {
            let ny = y + 1;
            let nid = ny * width + x;
            neighbors.push((nid, self.cells[nid].cur_resource()));
        }

        if x > 0 {
            let nx = x - 1;
            let nid = y * width + nx;
            neighbors.push((nid, self.cells[nid].cur_resource()));
        }

        if x + 1 < width {
            let nx = x + 1;
            let nid = y * width + nx;
            neighbors.push((nid, self.cells[nid].cur_resource()));
        }

        neighbors
    }

    fn allocate_resources(&mut self) {
        let mut cell_to_agents: Vec<Vec<usize>> = vec![Vec::new(); self.cells.len()];
        for (i, agent) in self.agents.iter().enumerate() {
            if !agent.is_alive() {
                continue;
            }
            let cid = agent.cid();
            cell_to_agents[cid].push(i);
        }

        for (cid, agent_indices) in cell_to_agents.iter().enumerate() {
            if agent_indices.is_empty() {
                continue;
            }

            let total = self.cells[cid].cur_resource();
            if total == 0 {
                continue;
            }

            let n = agent_indices.len() as u32;
            let base_share = total / n;
            let mut remaining = total - base_share * n;

            for &i in agent_indices {
                let leftover = self.agents[i].retrieve_resource(base_share);
                remaining += leftover;
            }
            let spent = total - remaining;
            let _ = self.cells[cid].take_up_to(spent);
        }
    }

    fn handle_agent_death(&mut self, id: usize) {
        let cid = self.agents[id].cid();
        let corpse_resource: u32 = 5;
        let regen_bonus: u32 = 1;

        self.cells[cid].add_resource(corpse_resource);
        self.cells[cid].increase_rate(regen_bonus);
    }

    fn step_agent(&mut self, id: usize) {
        if !self.agents[id].is_alive() {
            return;
        }

        if self.agents[id].is_hungry() {
            let cid = self.agents[id].cid();
            let neighbors = self.neighbor_cells_info(cid);

            if let Some(target_cid) = self.agents[id].decide_move(&neighbors) {
                let _ = self.agents[id].move_to(target_cid);
            }
        }

        if !self.agents[id].is_alive() {
            self.handle_agent_death(id);
            return;
        }

        let _ = self.agents[id].update();

        if !self.agents[id].is_alive() {
            self.handle_agent_death(id);
        }
    }

    fn step_all_agents(&mut self) {
        let len = self.agents.len();
        for id in 0..len {
            self.step_agent(id);
        }
    }
}

impl Updatable for World {
    fn update(&mut self) -> Result<(), SimulationError> {
        for cell in &mut self.cells {
            let _ = cell.update();
        }

        self.allocate_resources();
        self.step_all_agents();

        Ok(())
    }
}
