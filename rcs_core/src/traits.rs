use crate::errors::SimulationError;

/// Common interface for objects that advance one step in the simulation.
///
/// Types that participate in the simulation loop (such as `Cell`, `Agent`,
/// or `World`) implement this trait so they can be updated in a uniform way.
///
/// The exact meaning of “update” depends on the implementor:
/// - for a cell, it may regenerate resources
/// - for an agent, it may move and metabolize
/// - for the world, it may update all cells and agents
pub trait Updatable {
    /// Advances the object by one simulation step.
    ///
    /// ### Parameters
    /// This method takes `&mut self` and does not require extra parameters.
    ///
    /// ### Returns
    /// - `Ok(())` if the update succeeds.
    /// - `Err(SimulationError)` if the update fails for some reason
    ///   (for example, trying to update a dead agent).
    fn update(&mut self) -> Result<(), SimulationError>;
}
