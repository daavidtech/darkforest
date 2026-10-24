# Map

The Darkforest map is a coordinate system with sparse ground-truth objects and
separate civilization knowledge. The engine does not need to store every grid
cell as an object. Cells exist conceptually, while important state is stored as
objects, observations, and reports.

The core split is:

```text
Ground truth = what actually exists.
Knowledge = what a civilization knows, remembers, or estimates.
```

The UI mostly renders knowledge, not perfect truth.

## Ground Truth

Ground truth is authoritative engine state. It contains the real objects and
conditions of the world, whether or not any player can currently see them.

Ground truth can include:

- Scopes and grids
- Terrain seed or terrain overrides
- Civilizations
- Settlements, villages, cities, colonies, planets, and fleets
- Buildings
- Population groups
- Resources stored in settlements
- Army movements
- Hidden objects
- Anomalies and Epic Project sites

Example:

```text
Truth:
Enemy village at (80, 42)
Resources: 120 food, 60 wood
Population groups: 30 gatherers, 10 warriors
Buildings: Central Camp, Storage Pit
```

This truth may be completely unknown to another civilization.

## Civilization Knowledge

Knowledge is stored per civilization. It records what that civilization has
explored, observed, inferred, or received through reports.

Knowledge can include:

- Explored cells
- Currently observed cells
- Last-seen terrain
- Last-seen settlements
- Estimated population
- Estimated troop counts
- Detected movement
- Sensor contacts
- Scout reports
- Confidence levels
- Observation timestamps

Example:

```text
Knowledge:
Scout saw settlement at (80, 42) 9 hours ago.
Population estimate: 80-150.
Warriors: unknown.
Confidence: medium.
```

Knowledge can be true, incomplete, outdated, or approximate. Confirmed
observations are true at the time of observation. Estimates are guesses derived
from reports, sensors, confidence, and age.

## Knowledge States

The map has several knowledge states:

```text
Unknown
-> Explored
-> Observed
-> Controlled
```

Unknown areas have never been seen. Terrain, resources, players, buildings,
units, and paths are unknown.

Explored areas have been seen before but are not currently watched. Terrain
remains known, but buildings, units, and enemy activity become outdated. The map
shows last observed information with a timestamp and confidence.

Observed areas are currently visible through buildings, units, scouts, sensors,
or allies. Normal visible activity is updated, but hidden, stealthed,
underground, or advanced targets may still be missed.

Controlled areas are inside owned territory or strong sensor coverage. Alerts
are faster and normal movement is easier to detect, but control is still not
perfect against advanced stealth or unusual technologies.

The core rule is:

```text
Exploration reveals terrain.
Observation reveals current activity.
Intelligence estimates hidden truth.
```

## Sparse Cells

Cells do not need to be stored unless they have persistent state. The map can be
generated from a grid definition, terrain seed, and sparse object lists.

Store:

- Grid dimensions
- Topology
- Terrain seed
- Sparse terrain overrides
- Objects with positions and footprints
- Known cells per civilization
- Changed cells, such as roads, ruins, pollution, or damage

Do not store:

- One permanent object for every empty coordinate

When the UI draws a viewport, it asks the map for each visible coordinate:

```text
terrain = terrain_at(seed, position)
objects = objects_at(position)
knowledge = knowledge_at(civilization, position)
```

This keeps large maps cheap while still allowing local detail.

## Core Structures

The map model needs a few small shared structures. These structures describe
coordinate spaces and locations; domain objects like villages, buildings,
planets, fleets, and population groups can then use them.

### Scope

A scope is one coordinate space. A village interior, a local region, a planet
surface, a star system, and an interstellar map are all scopes.

```rust
pub struct Scope {
    pub name: String,
    pub kind: ScopeKind,
    pub grid: Grid,
    pub parent_location: Option<Location>,
}

pub enum ScopeKind {
    Village,
    Region,
    Planet,
    StarSystem,
    Interstellar,
}
```

`parent_location` describes where this scope exists inside another scope. For
example, a village scope can be inside a village object on the region scope, and
a planet surface scope can be inside a planet object in a star system scope.

### Grid

A grid describes the dimensions and topology rules for one scope.

```rust
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub topology: GridTopology,
    pub terrain_seed: u64,
}

pub enum GridTopology {
    Flat,
    WrapX,
    Torus,
    SphereLike,
}
```

Gameplay code should not manually hardcode edge behavior. It should ask the
grid for neighbors, normalized positions, and distance.

### Location

A location is a coordinate inside a specific scope.

```rust
pub struct Location {
    pub scope_id: ArenaId,
    pub x: i32,
    pub y: i32,
}
```

The `scope_id` is important because `x, y` alone is ambiguous. Coordinate
`(4, 7)` could mean a village tile, a region cell, a planet cell, or a star
system position.

```text
Central Camp: village scope (4, 4)
Village: region scope (50, 50)
Planet: star system scope (3, 0)
```

Shared systems such as movement, visibility, pathfinding, and viewport rendering
should use `Location` instead of separate loose `x` and `y` values.

### Footprint

A footprint describes the cells an object occupies inside its parent scope.

```rust
pub enum Footprint {
    SingleCell,
    Rectangle { width: u32, height: u32 },
    Cells(Vec<GridOffset>),
}

pub struct GridOffset {
    pub dx: i32,
    pub dy: i32,
}
```

An object can have an anchor `Location` and a `Footprint`. The anchor is the
main coordinate; the footprint is the occupied area relative to it.

### Domain Objects

Domain objects should stay specific even though they share map structures. A
village should be a village, not a generic settlement record that tries to also
model cities, planets, and fleets.

```rust
pub struct Village {
    pub civilization_id: ArenaId,
    pub name: String,
    pub location: Location,
    pub footprint: Footprint,
    pub interior_scope_id: ArenaId,
    pub resources: Resources,
}

pub struct Building {
    pub village_id: ArenaId,
    pub location: Location,
    pub kind: BuildingKind,
}
```

This keeps shared map behavior generic while still allowing village-specific,
city-specific, planet-specific, and fleet-specific rules.

### Knowledge Records

Knowledge records also use locations, but they belong to a civilization's known
view of the world, not directly to ground truth.

```rust
pub struct CivilizationKnowledge {
    pub civilization_id: ArenaId,
    pub known_cells: Vec<KnownCell>,
    pub known_objects: Vec<KnownObject>,
}

pub struct KnownCell {
    pub location: Location,
    pub state: KnowledgeState,
    pub last_seen_at: GameTime,
}

pub enum KnowledgeState {
    Explored,
    Observed,
    Controlled,
}
```

The engine can derive knowledge from ground truth through scouts, sensors,
reports, and alliances. The UI then renders this knowledge instead of rendering
perfect ground truth.

## Cell Scale

For world and planet-surface play, the default cell scale is:

```text
1 map cell = 1 km x 1 km
```

This means a whole hunter-gatherer camp or early village can fit inside one
cell. Larger settlements can occupy multiple cells through a footprint.

Examples:

```text
Hunter-gatherer camp: 1 cell
Village: 1-2 cells
Town: 2x2 or 3x3 cells
City: 5x5+ cells
Megacity or arcology: special large footprint
```

The cell is the terrain and movement unit. A village, city, or planet is an
object located on a grid, not the grid cell itself.

## Positions and Footprints

Objects can have an anchor position and a footprint.

```text
anchor = main coordinate
footprint = occupied cells relative to anchor
```

A simple footprint model:

```text
SingleCell
Rectangle(width, height)
Cells(offsets)
```

For example, a town with anchor `(50, 50)` and rectangle `3x2` occupies:

```text
(50,50), (51,50), (52,50)
(50,51), (51,51), (52,51)
```

This allows villages, cities, planetary colonies, megastructures, and strange
late-game objects to occupy more than one grid cell.

## Nested Scopes

The game has multiple map levels. A village can have its own interior view, and
later a planet can be one object among many in a star system.

Examples:

```text
Village View
Region View
Planet View
System View
Interstellar View
```

Each view is a scope. Objects can live inside a scope and can also contain a
child scope.

Example hierarchy:

```text
Interstellar scope
-> Star system object

Star system scope
-> Planet object

Planet scope
-> Region or city object

Region scope
-> Village object

Village scope
-> Central Camp, Shelter, Storage Pit
```

The same engine pattern can handle shelters inside a village, villages on a
regional map, cities on a planet, colonies on moons, fleets in star systems, and
Epic Projects in unusual spaces.

## Village and Settlement Views

The world map uses 1 km cells, but a village needs a zoomed-in view for building
placement and local management.

This creates two coordinate types:

```text
World position: position in a region or planet grid
Village position: local position inside a village scope
```

Village-internal buildings:

- Central Camp
- Shelter
- Storage Pit
- Ritual or teaching place

Nearby world-cell sites:

- Food gathering site
- Hunting camp
- Woodcutting site
- Stone working site

For MVP, the village view can be a small local grid such as `8x8` or `12x12`.
The exact local cell size can stay abstract at first.

## Topology

The grid cell concept stays the same across map types. What changes is topology:

- Which cells are neighbors
- How positions normalize across edges
- How distance is calculated
- How viewport panning works
- How movement routes are found

Topology examples:

```text
Flat
WrapX
Torus
SphereLike
Ring
Graph
Exotic
```

Flat topology has hard edges. It fits villages, local regions, and many
interior scopes.

WrapX topology wraps horizontally. It is a practical first model for planet
surfaces:

```text
Moving east past the right edge returns at the left edge.
Moving west past the left edge returns at the right edge.
North and south edges remain edges.
```

Torus topology wraps both axes. This is not a normal planet, but it can be a
useful topology for artificial habitats, simulations, alien spaces, or exotic
physics.

Sphere-like topology can model polar behavior more accurately. Crossing a pole
does not move directly to the opposite edge. Instead, longitude flips and travel
direction reverses. This is more realistic, but harder for UI and pathfinding.

For MVP:

```text
Village scope: Flat
Region scope: Flat
Planet scope: WrapX
```

The engine should avoid hardcoding direct edge checks into gameplay logic.
Movement, visibility, and viewport rendering should ask the scope topology for
neighbors, distance, and normalized positions.

## Planet View

Planet view can be represented as a rectangular grid with `WrapX` topology.
The player can pan east or west forever, while the engine stores one finite
grid.

Example:

```text
width = 1000
height = 500
camera center = (998, 120)
visible x coordinates = 996, 997, 998, 999, 0, 1, 2
```

The UI does not need special edge behavior. It asks topology helpers to
normalize positions before drawing cells.

Later, planets can become more realistic through cube-sphere, geodesic, or
sphere-like topologies if planet-scale gameplay needs it.

## Visibility Sources

Every owned building can have a sensor area. This is the area the building can
monitor around itself. A settlement naturally sees its surroundings, and remote
buildings or outposts extend map awareness.

Example early sensor ranges:

```text
Camp: 1 cell radius
Food Gathering Site: 1 cell radius
Shelter: 1 cell radius
Central Camp: 2-3 cell radius
Watchtower: 6 cell radius
Scout Tower: 8 cell radius
Signal Station: emission detection at 20+ cells
Observatory: orbital and space anomaly detection
```

Sensor range is not perfect truth. Each sensor can have:

- Range
- Detection types
- Confidence
- Refresh rate
- Stealth resistance

Detection types include:

- Visual detection for normal buildings and units
- Movement detection for armies and convoys
- Heat detection for industry and energy use
- Radio detection for communications and advanced settlements
- Orbital detection for satellites, stations, and fleets
- Gravitational detection for megastructures and unusual physics
- Quantum detection for very advanced anomalies

## Information Decay

Information decays over time:

```text
0-1 hours old: high confidence
1-12 hours old: medium confidence
12-48 hours old: low confidence
48+ hours old: stale or archival
```

Stale information can still be useful, but it must be marked as old. Players
make decisions from incomplete and outdated knowledge.

## Strategic Consequences

Advanced civilizations create more detectable signals:

- Hunter-gatherer camps are hard to detect at long range.
- Industrial cities create smoke, heat, logistics, and movement signatures.
- Radio civilizations create emissions.
- Nuclear and space civilizations create energy spikes and launch signatures.
- Epic Projects create abnormal strategic signatures.

Players can reduce visibility through concealment:

- Terrain cover
- Underground buildings
- Emission control
- Decoy settlements
- Camouflage
- Stealth technology
- Signal discipline
- Hidden colonies

This creates a strategic tradeoff:

```text
Build wide:
+ more sensor coverage
+ better warning
+ better logistics awareness
- more visible footprint
- more places to defend

Stay compact:
+ easier to hide
+ smaller signal footprint
+ easier defense
- less sensor coverage
- less warning time
- weaker map knowledge
```

The map never lies, but it can be incomplete, outdated, or ambiguous.
