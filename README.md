## 2D-Resource-Competition-Simulation
A Rust-based 2D simulation that models the **feedback loop** among resources, agents, and the environment.  
The project is split into a core simulation crate (`rcs_core`) and a GUI crate, and uses `eframe` / `egui` for visualization and interaction.

---

### What Was Built
- A **2D grid world** where each cell stores regenerating resources.
- A population of **agents** that:
  - compete for resources,
  - move to the neighboring cell with the largest available resource when hungry,
  - lose health and eventually die if they remain underfed.
- A simple **death feedback** mechanism:
  - when an agent dies, its cell gains additional resources and a small boost to regeneration rate.
- A **GUI application** that:
  - visualizes the grid and agents in real time,
  - exposes controls for pausing, stepping, resetting, and editing the world configuration.
- A set of **unit tests** in `rcs_core` that exercise the logic of `Cell`, `Agent`, and `World`.

---

### How to Run
From the project (workspace) root:
 - Build and run the GUI simulation: `cargo run`
 - Run the core simulation tests: `cargo test -p rcs_core`

---

### How It Works

#### Core Idea
On a 2D grid:
 - **Cells** represent environmental resource units.
 - **Agents** consume and move according to local resource availability.
 - **Death feedback** enriches the environment, closing a resource loop: dead agents “return” value to the cell, increasing both stored resource and regeneration rate.

---

### System Overview

#### Resource Field (Cells)
 - The background is a **grid of cells** of size `width x height`.
 - Each cell stores: 
   - `id`: a unique identifier (row-major index)
   - `cur_resource`: current available resource
   - `max_resource`: capacity limit
   - `regen_rate`: resource regeneration per update
   - `max_regen_rate`: upper bound on regeneration rate
 - Each simulation step:
   - `regen_rate` is added to `cur_resource`, capped at `max_resource`.
 - When an agent dies in a cell:
   - The cell receives a **fixed resource boost** (a small amount).
   - The `regen_rate` is increased by a small bonus, capped by `max_regen_rate`.

#### Agents
Each **agent** acts as a simple consumer and explorer.  
 - Agent stores:
   - `id`: a unique identifier
   - `cid`: id of the cell it currently occupies
   - `consumption_rate`: how much resource it needs per step
   - `allocated_resource`: the amount of resource currently allocated to it
   - `health_point`: its remaining health points
     - If `allocated_resource < consumption_rate`, HP is reduced by 1 on update.
     - If HP reaches 0, the agent is marked as dead and can no longer move or update.
   - `alive`: whether it is still alive
 - Movement:
   - If the agent is **hungry** (`allocated_resource < consumption_rate`), it looks at the four neighboring cells and chooses the one with the highest resource.
   - Moving to a new cell costs 1 HP (**movement cost**).
 - Death feedback:
   - When the agent dies, its current cell gains extra resource and a regen boost.

#### Resource Sharing & Simulation Loop
The world owns all cells and agents and implements a single `update` step via the `Updatable` trait.  
On each `World::update`:
 1. **Cell regeneration**  
    Every cell regenerates resource according to its regen_rate.
 2. **Resource allocation**
    For each cell:
     - Collect all **alive agents** currently in that cell.
     - Let `total` be the cell’s `cur_resource`.
     - Split `total` **equally** into `base_share = total / n` for `n` agents.
     - Each agent calls `retrieve_resource(base_share)` and takes up to its `consumption_rate`.
     - Any unused portion of each share is returned; the cell finally deducts only the amount that agents actually consumed.
 3. **Agent step**
    For each agent:
     - If dead, skip.
     - If hungry, look at neighboring cells via `neighbor_cells_info` and **optionally move** to the richest neighbor, paying a movement HP cost.
     - If the agent dies during movement, trigger **death feedback** for that cell.
     - if agent still alive, call `agent.update()`:
       - This runs metabolism: if underfed in last cell (if moved), HP decreases and the agent may die.
     - If the agent dies, trigger **death feedback** for current cell.

This `World::update` is called either automatically in the GUI (when not paused) or manually when the user presses Step.

---

### Interaction & Visualization

#### Interaction Features
 - **Reset**
   - Rebuilds the `World` from the current `WorldConfig`.
   - Resets the tick counter and simulation time.
 - **Pause / Resume**
   - Toggles automatic stepping.
 - **Step**
   - Advances the simulation by a single `World::update` step.
 - **View controls**
   - Adjusts the **cell size in pixels**.
 - **Simulation speed**
   - A slider for **seconds per tick**; when not paused, the world updates once per interval.
 - **World configuration panel**
   - Editable fields for:
     - Grid size: `width × height`
     - Cell initial resource range: `[min_resource, max_resource]`
     - Cell regeneration rate range: `[min_regen_rate, max_regen_rate]`
     - Agent count range: `[min_agents, max_agents]`
     - Agent consumption range: `[min_consumption_rate, max_consumption_rate]`
     - Agent HP (initial, fixed)
   - The actual initialization uses **uniform random sampling** within these ranges when you hit **Reset**.

#### Visualization
 - **Cells**
   - Drawn as rectangles in a grid.
   - Fill color encodes the **current resource level** relative to `max_resource`: richer cells appear brighter / more saturated.
   - Each cell also has a dark border to keep the grid visually clear.
 - **Agents**
   - Drawn as circles centered inside their current cells.
   - Color encodes **health**:
     - full HP -> close to white,
     - low HP -> more red.
 - **UI**
   - Built with `eframe` / `egui` in immediate-mode style.

---

### What Didn’t Work
 - **Simplified agent decision-making**
   - The original idea included more nuanced behaviors (e.g., different strategies when “slightly hungry” vs “severely starving”).
   - In the final implementation, the rule is simpler:  
   an agent either has enough food (stays put) or is hungry and always greedily moves toward the richest neighbor, if any.
 - **Visualization for multiple agents in same cell**
   - When multiple agents occupy the same cell, the visual effect is no different from having a single agent in that cell.
   - The colors of the other agents are also overwritten by the agent on the top.
 - **Purely local view**
   - Agents only see four neighbors (up, down, left, right).
   - There is no long-range planning or pathfinding, so agents can make shortsighted moves and still starve.
   - When all surrounding resources are same as the current resource, the agent moves back and forth.
 - **Allocateion problem**
   - Since the current resource allocation logic distributes resources evenly by default, when multiple agents occupy the same cell, they are usually 'underfed', which causes all of them to move outward.
   - In addition, even when the allocated resources are insufficient to cover consumption, agents will still 'eat' rather than allowing other agents to be fully fed first. In other words, all agents end up underfed.
 - **Configuration changes require reset**
   - Editing the world config in the side panel does not modify the current world in-place; changes only take effect after pressing **Reset**.
   - This keeps the implementation simple but limits “live tuning” of parameters on a running world.

---

### Lessons Learned
 - **Separation of concerns in Rust**
   - Splitting the project into a core crate (`rcs_core`) and a GUI crate helped keep the simulation logic independent of the UI framework.
 - **Using traits for simulation steps**
   - Defining an `Updatable` trait with a single `update()` method made it natural to give a common “step” interface to `Cell`, `Agent`, and `World`.
 - **Define the responsibilities of each 'role' in advance**
   - Having a clear responsibility structure from the beginning helps streamline future debugging and optimization of the logic.
 - **Error handling instead of panics**
   - Introducing a `SimulationError` enum (e.g., `NotAlive`, `NotEnoughResources`) made APIs like `Agent::move_to` and `Cell::resource_consumption` safer and more explicit about failure cases.
 - **Saturating arithmetic matters**
   - Using `saturating_sub` and clamping values to `max_resource` / `max_regen_rate` avoided underflow and unbounded growth in the simulation.

---

### AI Usage
 - The implementation of the egui components was developed with assistance and guidance from **ChatGPT**.
 - Documentation comments were generated by **ChatGPT** and then reviewed independently.
 - Unit tests were added later and were also written with help from **ChatGPT**.

---

### Notes
In the current implementation, `world::neighbor_cells_info` collects neighboring cells in the order “**up**, **down**, **left**, **right**,” while `agent::decide_move` selects the last neighbor in the list when their resource values are equal. Consequently, when all cells have the same (and relatively low) amount of resources, agents tend to move to the right, producing a neat, synchronized pattern of motion. Different world configurations give rise to different spatial patterns (with agents eventually coming to rest visually), but these differences essentially stem from the perturbations introduced by randomness; once randomness is removed, the simulation becomes almost completely predictable and thus rather monotonous.