use crate::errors::SimulationError;
use crate::traits::Updatable;

/// A single cell in the world grid, storing resources and a regeneration rate.
///
/// Each `Cell` has:
/// - its unique `id`
/// - the current amount of resource available
/// - the maximum amount of resource it can hold
/// - its current regeneration rate per update step
/// - the maximum regeneration rate it can reach
pub struct Cell {
    id: usize,
    cur_resource: u32,
    max_resource: u32,
    regen_rate: u32,
    max_regen_rate: u32,
}

impl Cell {
    /// Creates a new cell with given parameters.
    ///
    /// ### Parameters
    /// - `id`: Unique identifier of this cell in the world.
    /// - `cur_resource`: Initial resource amount stored in the cell.
    /// - `max_resource`: Maximum resource capacity of the cell.
    /// - `regen_rate`: Initial regeneration rate per update step.
    /// - `max_regen_rate`: Upper bound on the regeneration rate.
    ///
    /// ### Returns
    /// A new [`Cell`] instance with the given configuration.
    pub fn new(
        id: usize,
        cur_resource: u32,
        max_resource: u32,
        regen_rate: u32,
        max_regen_rate: u32,
    ) -> Self {
        Self {
            id,
            cur_resource,
            max_resource,
            regen_rate,
            max_regen_rate,
        }
    }

    /// Adds resource to the cell, saturating at `max_resource`.
    ///
    /// ### Parameters
    /// - `resource`: Amount of resource to add.
    ///
    /// ### Returns
    /// This method does not return a value. It updates the internal
    /// `cur_resource` field in place.
    pub fn add_resource(&mut self, resource: u32) {
        self.cur_resource = (self.cur_resource.saturating_add(resource)).min(self.max_resource);
    }

    /// Consumes an exact amount of resource from the cell.
    ///
    /// If the cell does not contain enough resource, an error is returned
    /// and the internal state is left unchanged.
    ///
    /// ### Parameters
    /// - `resource`: Desired amount of resource to consume.
    ///
    /// ### Returns
    /// - `Ok(amount)` if the cell contained at least `amount` resource.
    /// - `Err(SimulationError::NotEnoughResources { .. })` if there is not enough.
    pub fn resource_consumption(&mut self, resource: u32) -> Result<u32, SimulationError> {
        if self.cur_resource < resource {
            return Err(SimulationError::NotEnoughResources {
                available: self.cur_resource,
            });
        }
        self.cur_resource -= resource;
        Ok(resource)
    }

    /// Takes up to a requested amount of resource from the cell.
    ///
    /// Unlike [`Cell::resource_consumption`], this method never fails:
    /// it simply returns the minimum of the requested amount and the
    /// available resource.
    ///
    /// ### Parameters
    /// - `want`: Requested amount of resource.
    ///
    /// ### Returns
    /// The actual amount of resource taken from the cell.
    pub fn take_up_to(&mut self, want: u32) -> u32 {
        let take = want.min(self.cur_resource);
        self.cur_resource -= take;
        take
    }

    /// Increases the regeneration rate, saturating at `max_regen_rate`.
    ///
    /// ### Parameters
    /// - `regen_rate`: Increment to add to the current regeneration rate.
    ///
    /// ### Returns
    /// This method does not return a value. It updates `regen_rate` in place.
    pub fn increase_rate(&mut self, regen_rate: u32) {
        self.regen_rate = (self.regen_rate.saturating_add(regen_rate)).min(self.max_regen_rate);
    }

    /// Returns the id of the cell.
    ///
    /// ### Returns
    /// The unique identifier of this cell.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the current amount of resource stored in the cell.
    ///
    /// ### Returns
    /// The current resource amount.
    pub fn cur_resource(&self) -> u32 {
        self.cur_resource
    }
}

impl Updatable for Cell {
    /// Regenerates the cell's resource by its current `regen_rate`,
    /// saturating at `max_resource`.
    ///
    /// ### Returns
    /// - `Ok(())` on success.  
    ///   Currently this implementation never fails, but the `Result`
    ///   type allows future error handling (e.g., invalid configuration).
    fn update(&mut self) -> Result<(), SimulationError> {
        self.cur_resource =
            (self.cur_resource.saturating_add(self.regen_rate)).min(self.max_resource);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::SimulationError;

    #[test]
    fn new_initializes_fields_correctly() {
        let cell = Cell::new(1, 10, 100, 2, 5);
        assert_eq!(cell.id(), 1);
        assert_eq!(cell.cur_resource(), 10);
    }

    #[test]
    fn add_resource_caps_at_max_resource() {
        let mut cell = Cell::new(0, 90, 100, 1, 5);
        cell.add_resource(20);
        assert_eq!(cell.cur_resource(), 100);
    }

    #[test]
    fn resource_consumption_succeeds_when_enough_resource() {
        let mut cell = Cell::new(0, 10, 100, 1, 5);
        let taken = cell.resource_consumption(4).unwrap();
        assert_eq!(taken, 4);
        assert_eq!(cell.cur_resource(), 6);
    }

    #[test]
    fn resource_consumption_fails_when_not_enough_resource() {
        let mut cell = Cell::new(0, 3, 100, 1, 5);
        let err = cell.resource_consumption(5).unwrap_err();

        assert!(matches!(
            err,
            SimulationError::NotEnoughResources { available } if available == 3
        ));
        assert_eq!(cell.cur_resource(), 3);
    }

    #[test]
    fn take_up_to_takes_at_most_requested_amount() {
        let mut cell = Cell::new(0, 10, 100, 1, 5);
        let taken = cell.take_up_to(4);
        assert_eq!(taken, 4);
        assert_eq!(cell.cur_resource(), 6);
    }

    #[test]
    fn take_up_to_takes_all_when_request_exceeds_available() {
        let mut cell = Cell::new(0, 5, 100, 1, 5);
        let taken = cell.take_up_to(10);
        assert_eq!(taken, 5);
        assert_eq!(cell.cur_resource(), 0);
    }

    #[test]
    fn increase_rate_caps_at_max_regen_rate() {
        let mut cell = Cell::new(0, 0, 100, 1, 5);
        cell.increase_rate(10);
        cell.update().unwrap();
        assert_eq!(cell.cur_resource(), 5);
    }

    #[test]
    fn update_regenerates_resource_but_not_over_max() {
        let mut cell = Cell::new(0, 99, 100, 5, 5);
        cell.update().unwrap();
        assert_eq!(cell.cur_resource(), 100);
    }
}
