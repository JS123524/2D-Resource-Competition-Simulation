use crate::errors::SimulationError;
use crate::traits::Updatable;

/// An agent that moves between cells and consumes resources to stay alive.
///
/// Each `Agent` has:
/// - a unique `id`
/// - the id of the cell it currently occupies (`cid`)
/// - its per-step consumption rate
/// - the amount of resource currently allocated to it
/// - its remaining health points
/// - whether it is still alive
pub struct Agent {
    id: usize,
    cid: usize,
    consumption_rate: u32,
    allocated_resource: u32,
    health_point: u32,
    alive: bool,
}

impl Agent {
    /// Creates a new agent with the given parameters.
    ///
    /// ### Parameters
    /// - `id`: Unique identifier of this agent.
    /// - `cid`: Id of the cell where the agent starts.
    /// - `consumption_rate`: Resource needed per update step to avoid health loss.
    /// - `allocated_resource`: Resource currently allocated to the agent.
    /// - `health_point`: Initial health points of the agent.
    /// - `alive`: Initial alive status.
    ///
    /// ### Returns
    /// A new [`Agent`] instance.
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

    /// Performs one step of metabolism for the agent.
    ///
    /// If `allocated_resource` is smaller than `consumption_rate`, the agent
    /// loses one health point. In all cases, `allocated_resource` is reset
    /// to zero. When `health_point` reaches zero, `alive` is set to `false`.
    ///
    /// This method is internal; external callers should use [`Agent::update`].
    fn metabolize(&mut self) {
        if self.allocated_resource < self.consumption_rate {
            self.health_point = self.health_point.saturating_sub(1);
        }
        self.allocated_resource = 0;
        if self.health_point == 0 {
            self.alive = false;
        }
    }

    /// Applies the movement cost to the agent.
    ///
    /// Movement always costs one health point (saturating at zero). If the
    /// health reaches zero, the agent is marked as dead.
    ///
    /// This method is internal; external callers should use [`Agent::move_to`].
    fn movement_cost(&mut self) {
        self.health_point = self.health_point.saturating_sub(1);
        if self.health_point == 0 {
            self.alive = false;
        }
    }

    /// Moves the agent to a new cell, applying a movement cost.
    ///
    /// If the agent is already dead, this method returns an error and leaves
    /// its state unchanged.
    ///
    /// ### Parameters
    /// - `new_id`: The id of the cell to move to.
    ///
    /// ### Returns
    /// - `Ok(())` if the agent is alive and the move succeeds.
    /// - `Err(SimulationError::NotAlive)` if the agent is dead.
    pub fn move_to(&mut self, new_id: usize) -> Result<(), SimulationError> {
        if !self.alive {
            return Err(SimulationError::NotAlive);
        }
        self.cid = new_id;
        self.movement_cost();
        Ok(())
    }

    /// Retrieves resource for the agent from a cell's available amount.
    ///
    /// The agent takes up to its `consumption_rate` from the given `resource`
    /// pool. The amount actually taken is stored in `allocated_resource`, and
    /// the remaining resource (if any) is returned.
    ///
    /// ### Parameters
    /// - `resource`: Total amount of resource offered to the agent.
    ///
    /// ### Returns
    /// The leftover resource that was not taken by the agent.
    pub fn retrieve_resource(&mut self, resource: u32) -> u32 {
        let take = resource.min(self.consumption_rate);
        self.allocated_resource = take;
        resource - take
    }

    /// Decides which neighboring cell to move to based on available resources.
    ///
    /// The agent selects the neighbor with the largest resource value.
    /// Cells with zero resource are ignored; if all neighbors have zero
    /// or the slice is empty, this method returns `None`.
    ///
    /// ### Parameters
    /// - `neighbor_cells`: A slice of `(cell_id, resource)` pairs describing
    ///   the agent's neighboring cells.
    ///
    /// ### Returns
    /// - `Some(cell_id)` for the chosen destination.
    /// - `None` if there is no beneficial move.
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

    /// Returns the id of this agent.
    ///
    /// ### Returns
    /// The unique identifier of the agent.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the id of the cell the agent currently occupies.
    ///
    /// ### Returns
    /// The id of the current cell.
    pub fn cid(&self) -> usize {
        self.cid
    }

    /// Returns whether the agent is currently alive.
    ///
    /// ### Returns
    /// `true` if the agent is alive, `false` otherwise.
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// Returns whether the agent is hungry in this step.
    ///
    /// An agent is considered hungry if its `allocated_resource` is
    /// strictly less than its `consumption_rate`.
    ///
    /// ### Returns
    /// `true` if the agent is hungry, `false` otherwise.
    pub fn is_hungry(&self) -> bool {
        self.allocated_resource < self.consumption_rate
    }

    /// Returns the current health points of the agent.
    ///
    /// ### Returns
    /// The remaining health points.
    pub fn health_point(&self) -> u32 {
        self.health_point
    }
}

impl Updatable for Agent {
    /// Advances the agent by one simulation step.
    ///
    /// The agent must be alive; otherwise an error is returned.
    /// Internally, this calls `metabolize`, which may reduce
    /// health and potentially kill the agent if it remains underfed.
    ///
    /// ### Returns
    /// - `Ok(())` if the agent was alive and the update succeeded.
    /// - `Err(SimulationError::NotAlive)` if the agent is already dead.
    fn update(&mut self) -> Result<(), SimulationError> {
        if !self.alive {
            return Err(SimulationError::NotAlive);
        }
        self.metabolize();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::SimulationError;

    #[test]
    fn new_initializes_fields_correctly() {
        let a = Agent::new(1, 2, 3, 4, 5, true);
        assert_eq!(a.id(), 1);
        assert_eq!(a.cid(), 2);
        assert_eq!(a.health_point(), 5);
        assert!(a.is_alive());
    }

    #[test]
    fn retrieve_resource_takes_up_to_consumption_rate() {
        let mut a = Agent::new(0, 0, 3, 0, 5, true);

        // more resource than needed
        let leftover = a.retrieve_resource(10);
        assert_eq!(a.allocated_resource, 3);
        assert_eq!(leftover, 7);
        assert!(!a.is_hungry());

        // less resource than needed
        let leftover = a.retrieve_resource(2);
        assert_eq!(a.allocated_resource, 2);
        assert_eq!(leftover, 0);
        assert!(a.is_hungry());
    }

    #[test]
    fn metabolize_does_not_reduce_health_when_fed() {
        let mut a = Agent::new(0, 0, 3, 3, 5, true);
        a.update().unwrap();
        assert_eq!(a.health_point(), 5);
        assert_eq!(a.allocated_resource, 0);
        assert!(a.is_alive());
    }

    #[test]
    fn metabolize_reduces_health_when_hungry() {
        let mut a = Agent::new(0, 0, 3, 0, 5, true);
        a.update().unwrap();
        assert_eq!(a.health_point(), 4);
        assert_eq!(a.allocated_resource, 0);
        assert!(a.is_alive());
    }

    #[test]
    fn metabolize_kills_agent_at_zero_health() {
        let mut a = Agent::new(0, 0, 3, 0, 1, true);
        a.update().unwrap();
        assert_eq!(a.health_point(), 0);
        assert!(!a.is_alive());
    }

    #[test]
    fn update_fails_when_agent_is_dead() {
        let mut a = Agent::new(0, 0, 3, 0, 0, false);
        let err = a.update().unwrap_err();
        assert!(matches!(err, SimulationError::NotAlive));
    }

    #[test]
    fn move_to_changes_cell_and_applies_movement_cost() {
        let mut a = Agent::new(0, 1, 3, 0, 5, true);
        a.move_to(2).unwrap();
        assert_eq!(a.cid(), 2);
        assert_eq!(a.health_point(), 4);
        assert!(a.is_alive());
    }

    #[test]
    fn move_to_fails_if_agent_is_dead() {
        let mut a = Agent::new(0, 1, 3, 0, 0, false);
        let result = a.move_to(2);
        assert!(matches!(result, Err(SimulationError::NotAlive)));
        assert_eq!(a.cid(), 1);
    }

    #[test]
    fn decide_move_picks_neighbor_with_highest_resource() {
        let a = Agent::new(0, 0, 3, 0, 5, true);
        let neighbors = vec![(1, 1), (2, 10), (3, 5)];
        let target = a.decide_move(&neighbors);
        assert_eq!(target, Some(2));
    }

    #[test]
    fn decide_move_returns_none_when_no_resource_or_empty() {
        let a = Agent::new(0, 0, 3, 0, 5, true);

        let neighbors_all_zero = vec![(1, 0), (2, 0)];
        assert_eq!(a.decide_move(&neighbors_all_zero), None);

        let empty: Vec<(usize, u32)> = Vec::new();
        assert_eq!(a.decide_move(&empty), None);
    }
}
