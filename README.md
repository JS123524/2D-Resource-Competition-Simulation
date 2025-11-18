## 2D-Resource-Competition-Simulation
A Rust-based 2D simulation that models the **feedback loop** among resources, agents, and the environment.  
The project uses `eframe`/`egui` for visualization and interaction.

---

### Project Vision

#### Core Idea
On a 2D grid:
 - **Cells** represent environmental resource units.
 - **Agents** consume and move according to local resource availability.
 - **Death feedback** enriches the environment, closing the resource loop.

---

### System Overview

#### Resource Field
 - The background is a **grid of cells**.
 - Each cell holds:
   - `current_resource`: current available resources.
   - `max_resource`: storage capacity limit.
   - `regen_rate`: resource regeneration rate over time.
   - `max_regen_rate`: upper bound for regeneration speed.
 - When an agent dies in a cell:
   - The cell immediately receives a **resource boost**.
   - Its `regen_rate` slightly increases (both effects are bounded to prevent runaway growth).

#### Agents
Each **agent** acts as an autonomous consumer and explorer.
 - **Consumption**: the amount of resource required per step to stay alive.
 - **Harvesting**: the amount of resource it can extract from its current cell (affected by competition).
 - **Health**: decreases when harvesting is below consumption for too long.
 - **Death feedback**: upon death, the agent’s remaining value is recycled into its current cell.

#### Behavior
 - If harvesting < consumption (slightly): agent explores nearby cells to find richer resources.
 - If harvesting << consumption: agent remains idle, awaiting recovery or death.
 - Multiple agents in the same cell share resources equally.

---

### Interaction Features
 - **Spawn control**: user can choose the number of initial agents.
 - **Focus agent**: the first alive agent in the list is adjustable in real time (consumption and harvesting rate).
 - **Force kill button**: triggers an immediate death of the focus agent to demonstrate feedback.
 - **Reset**: reinitialize the world and all parameters.

---

### Visualization
 - **Cells**: drawn as rectangles.
   - **Color** intensity or numeric display indicates resource level.
 - **Agents**: drawn as small points.
 - **UI**: built using `eframe` / `egui`.

---

### World Model
| Category | Property | Description |
|----------|----------|-------------|
| **Cell** | Current Resource | Amount of allocatable resources available. |
|          | Max Resource | Maximum resources a cell can store. |
|          | Regen Rate | Regeneration rate per update step. |
|          | Max Regen Rate | Maximum regeneration rate (increases upon death). |
|          | Consumption Rate | Total consumption by all agents in the cell. |
| **Agent** | Consumption | Resource needed per step for survival. |
|           | Harvesting | Resource actually obtained (may differ from consumption). |
|           | Health | Decreases if harvesting remains too low. |
|           | State | Alive / Dead. Dead agents return resources to the cell. |
| **World** | Current Agents | Number of alive agents. |
|           | Max Agents | Maximum allowed agents. |
|           | Cells | The collection of resource cells forming the grid. |
