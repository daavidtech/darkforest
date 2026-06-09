# Darkforest

Darkforest is a Travian-style persistent strategy game with flavors from the
Three-Body Problem book series.

The main concepts are:

- Asymmetric technological evolution between players
- Real-world time passage, similar to Travian
- Uncertainty about other players through fog of war

Darkforest avoids a single obvious winning strategy. Early military expansion
can provide resources and map control, but it slows innovation by consuming
population, production, maintenance, and specialized buildings. A civilization
that appears weak militarily may be investing in science, automation, sensors,
or weapons that create a large asymmetric advantage later.

Military strength is easier to see than technological strength. The game rewards
players for making uncertain strategic reads: when to raid, when to hide, when
to invest, when to reveal strength, and when to pursue a technological leap.

## Table of Contents

- [Innovation](#innovation)
- [Population](#population)
- [Epic Projects](#epic-projects)
- [Time](#time)
- [Mailbox and Reports](#mailbox-and-reports)
- [Map Visibility](#map-visibility)
- [Technology Evolution](#technology-evolution)
- [Starting Conditions and Early Progression](#starting-conditions-and-early-progression)
- [MVP Pages](#mvp-pages)
- [Unit](#unit)
- [Battles](#battles)
- [Currency](#currency)
- [Map](#map)
- [Speed](#speed)
- [Building](#building)

## Innovation

Innovation score describes how likely a civilization is to discover, improve, or
unlock technologies. It is the strategic counterweight to pure army building.

Innovation grows from the health and structure of the civilization:

- Population size
- Population happiness
- Schools, universities, laboratories, and other education buildings
- Teachers, researchers, engineers, and other specialized units
- Social stability
- Resource surplus
- Communication and information infrastructure

Innovation is reduced by conditions that push society toward survival or war
instead of discovery:

- Large standing armies
- Military maintenance costs
- Assigning too much population to soldiers instead of civilian roles
- Starvation or resource shortages
- Low happiness
- Damaged infrastructure
- War exhaustion

This means early military expansion can provide resources, map control, and
security, but it also creates an opportunity cost. A civilization that invests in
innovation may be vulnerable at first, but later it can unlock asymmetric
advantages that make older armies obsolete.

Technology discovery is partly deterministic and partly probabilistic. Each
technology has requirements, but a higher innovation score increases the chance
and speed of discovering available technologies. Some technologies require
specific buildings, specialists, resources, or previous discoveries.

## Population

Population is the civilization's human capacity. It does not behave like a
normal resource that is simply produced and spent. Population provides workers,
soldiers, teachers, researchers, engineers, builders, farmers, and future
specialists.

Population growth follows an S-curve instead of growing linearly forever. A
civilization with poor food, housing, health, and stability grows slowly or may
decline. As food security, medicine, housing, and social stability improve,
population can grow rapidly. Later, advanced societies may stabilize as
urbanization, education, lifestyle, and opportunity cost reduce birth rates.

The goal is not to model real demographics perfectly, but to create strategic
pressure: population is power, but it needs food, housing, happiness, health,
and social conditions to grow.

Population can be modeled as:

- Total population
- Available population
- Assigned population
- Population cap or carrying capacity
- Happiness
- Health

Available population is the part of the population not currently assigned to a
specialized role. Training a soldier assigns population to the military instead
of spending it like wood or iron. If the unit dies, that population is lost. If
the unit is disbanded, some population may return to civilian life.

Assigned population can include:

- Soldiers
- Farmers
- Builders
- Teachers
- Researchers
- Engineers
- Doctors

For an early implementation, only total population, available population,
soldiers, population cap, and happiness are needed.

Population growth can start from a simple model:

```text
populationGrowth =
  totalPopulation
  * baseGrowthRate
  * foodFactor
  * housingFactor
  * healthFactor
  * happinessFactor
  * developmentStageFactor
  * carryingCapacityFactor
```

The carrying capacity factor slows growth as population approaches the current
capacity:

```text
carryingCapacityFactor = 1 - totalPopulation / carryingCapacity
```

Carrying capacity is increased by buildings, resources, infrastructure, and
technology:

- Food production
- Housing
- Sanitation
- Medicine
- Storage
- Territory
- Logistics
- Energy
- Automation

Each technological scale can create a new carrying capacity ceiling:

```text
Village carrying capacity
-> Regional carrying capacity
-> Planetary carrying capacity
-> Orbital carrying capacity
-> Multi-planetary carrying capacity
-> Interstellar carrying capacity
```

Multi-planetary expansion is a major carrying capacity breakthrough. It allows a
civilization to distribute population, industry, and resources across multiple
worlds. This makes the civilization harder to destroy because it no longer has
only one homeworld, one industrial base, and one obvious target.

However, expansion also creates exposure:

- More signals and emissions
- More logistics complexity
- More detectable movement
- More political fragmentation
- More places to defend
- Larger attack surface

This creates a dark forest strategic dilemma:

```text
Stay compact:
+ easier to hide
+ easier to defend
+ lower signal footprint
- limited population and industry
- vulnerable to one decisive strike

Expand multi-planetary:
+ huge carrying capacity
+ redundant population and industry
+ more resources
+ better long-term survival
- more visible
- harder to coordinate
- more frontier risk
```

## Epic Projects

Epic Projects are alliance-scale endgame objectives. They require massive
resources, high innovation, advanced technology, specialized workers, long
real-world build times, and coordinated defense.

Epic Projects are too large for one player to complete alone. They create
strategic pressure for alliances, supply chains, scouting, secrecy, sabotage,
and large-scale wars.

Each round can have different Epic Projects. The project type, purpose, recipe,
location, risks, and activation requirements are not fully revealed at the
start. Players discover them through exploration, research, anomalies, ancient
structures, rare events, high innovation, or contact with other civilizations.

An Epic Project can be a dimensional portal, interstellar ark, artificial star,
dark forest beacon, reality engine, civilization archive, or something unique to
that round.

Epic Projects turn the endgame into a mystery. Players are not only competing to
build the final objective; they are competing to understand what the objective
is, what it means, and whether activating it is safe.

Project knowledge can be discovered as fragments:

- Project name
- Rough purpose
- Required technology branch
- Special resource
- Build location constraints
- Activation risk
- Final construction phase

Example discovery chain:

```text
Large-scale spacetime manipulation is possible.
-> Stable portal construction requires negative-mass material.
-> Negative-mass deposits only appear near dead stars.
-> Portal activation creates a detectable universe-wide signal.
```

This makes the endgame feel like solving the round's cosmic purpose. A round may
not reveal from the beginning whether the goal is escape, survival, domination,
transcendence, or discovery.

## Time

Darkforest uses persistent real-world time. The game continues while players are
offline, and time itself is part of the strategy.

The civilization-scale clock is compressed:

```text
1 real day = 1 game year
```

This scale lets a season contain civilization-level change. A three-month round
represents about 90 game years, and a six-month round represents about 180 game
years. Population growth, technological eras, long-term wars, and Epic Projects
all fit inside one playable season.

The civilization clock is not applied literally to every action. Operational
actions use balanced real-world durations:

- Building construction takes seconds, minutes, hours, or days depending on
  scale.
- Unit training takes minutes or hours.
- Army and resource movement takes real travel time.
- Research discovery checks happen over real elapsed time.
- Epic Project phases take days or weeks.

Movement uses its own compressed physical scale:

```text
1 real minute = 1 game hour for movement
```

For example, a unit moving 8 km/h travels 8 km per real minute. Travel remains
strategically meaningful without forcing players to wait for literal historical
movement times.

Early game actions are fast enough for the first session to feel active:

```text
Basic building: 30 seconds to 10 minutes
Basic unit: 1 to 15 minutes
Nearby travel: 5 to 30 minutes
Early tech discovery: 1 to 12 hours
```

Mid-game actions take longer:

```text
Advanced building: 30 minutes to 12 hours
Specialized unit: 15 minutes to 6 hours
Regional travel: 30 minutes to 12 hours
Major tech discovery: 6 hours to 3 days
```

Late-game actions become strategic commitments:

```text
Mega structure: 1 to 7 days
Space unit or fleet: 6 hours to 3 days
Interplanetary travel: 6 hours to 3 days
Major breakthrough: 1 to 7 days
```

Endgame projects are alliance-scale time investments:

```text
Epic Project phase: 2 to 10 days
Full Epic Project: 1 to 4 weeks
```

Technology improves time efficiency. Better tools reduce construction time,
logistics reduces delivery time, automation reduces production bottlenecks,
education improves research speed, and transportation technology reduces
movement time.

## Mailbox and Reports

Each player has a mailbox that contains all reports, alerts, discoveries, and
messages known to their civilization. The mailbox is not only for player
messages. It is the main event feed for persistent time.

Reports come from buildings, units, technologies, and other players. A
civilization only receives reports for things it has the ability to know.

Examples:

- A scout returns and creates a scouting report.
- A university discovers technology and creates a discovery report.
- A farm storage fills and creates a production warning.
- A watchtower detects an incoming attack and creates a defense alert.
- A trade caravan arrives and creates a logistics report.
- A sensor station detects unusual emissions and creates an intelligence report.
- An Epic Project creates a detectable anomaly and creates a strategic report.
- Another player sends a diplomatic message.

The mailbox starts simple in the early game:

- Building completed
- Unit trained
- Scout returned
- Player message received

As the civilization develops, buildings and technologies increase the quality,
range, and type of reports:

- Town Hall creates basic administrative reports.
- Scout Camp creates scouting reports.
- School and University create research discovery reports.
- Watchtower creates local attack warnings.
- Signal Station creates distant emission reports.
- Intelligence Agency creates spy reports and confidence estimates.
- Observatory creates space and orbital anomaly reports.
- Advanced sensors create late-game hidden project clues.

Reports can include:

- Report type
- Time received
- Source
- Location, if known
- Confidence
- Last observed time
- Action links

Example report:

```text
Scout Report
Source: Scout Unit #184
Observed: 2 hours ago
Received: now
Confidence: medium

Settlement detected at 18,42.
Visible buildings: Farm, Barrack, unknown large structure.
Estimated soldiers: 20-50.
```

The mailbox is how uncertain knowledge enters the player interface. Players do
not automatically see perfect truth on the map. They receive reports, estimates,
warnings, and clues, then make decisions from incomplete information.

## Map Visibility

Map visibility is based on what the civilization can currently observe, what it
has explored before, and what it can infer from reports.

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
shows the last observed information with a timestamp and confidence.

Observed areas are currently visible through buildings, units, scouts, sensors,
or allies. Normal visible activity is updated in real time, but hidden,
stealthed, underground, or advanced targets may still be missed.

Controlled areas are inside owned territory or strong sensor coverage. Alerts
are faster and normal movement is easier to detect, but control is still not
perfect against advanced stealth or unusual technologies.

The core rule is:

```text
Exploration reveals terrain.
Observation reveals current activity.
Intelligence estimates hidden truth.
```

Every owned building has a sensor area. This represents the area the building
can monitor around itself. A settlement naturally sees its surroundings, and
remote buildings or outposts extend map awareness.

Example early sensor ranges:

```text
Farm: 1 tile radius
House: 1 tile radius
Barrack: 2 tile radius
Town Hall: 3 tile radius
Watchtower: 6 tile radius
Scout Tower: 8 tile radius
Signal Station: emission detection at 20+ tiles
Observatory: orbital and space anomaly detection
```

Sensor range is not the same as perfect truth. Each sensor has properties:

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

Different buildings detect different things. A farm has weak local visual
monitoring. A watchtower detects nearby movement well. A signal station may not
see soldiers in a forest, but it can detect distant radio emissions. An
observatory can detect orbital construction or space travel but may know little
about ground-level activity.

Scouts and buildings create different kinds of visibility:

- Buildings provide persistent local observation.
- Scouts provide mobile exploration and return reports.
- Watchtowers provide defensive warning.
- Signal stations detect distant activity indirectly.
- Observatories detect orbital and space-scale events.
- Intelligence buildings improve confidence and hidden activity estimates.

Information decays over time:

```text
0-1 hours old: high confidence
1-12 hours old: medium confidence
12-48 hours old: low confidence
48+ hours old: stale or archival
```

Advanced civilizations create more detectable signals:

- Primitive villages are hard to detect at long range.
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

The map never lies, but it can be incomplete, outdated, or ambiguous. Confirmed
information is true. Estimated information is a guess based on reports,
sensors, confidence, and last observation time.

## Technology Evolution

Technology evolution is asymmetric. Players do not all follow one fixed linear
technology tree. Civilizations discover different paths based on innovation,
buildings, specialists, resources, environment, events, and previous
discoveries.

Two civilizations can be equally advanced while being dangerous in completely
different ways. One may have strong weapons, another may have better logistics,
another may have better sensors, another may have superior stealth, and another
may have better population growth or industrial automation.

Technology has broad stages:

```text
Primitive
-> Agricultural
-> Iron
-> Industrial
-> Information
-> Nuclear
-> Space
-> Multi-planetary
-> Interstellar
-> Exotic physics
```

Stages describe the rough scale of a civilization, but they do not reveal the
exact technologies a player has. A civilization in the Industrial stage may be
focused on factories, logistics, artillery, medicine, education, or early
signals. A Space-stage civilization may be focused on orbital sensors, colonies,
fleet construction, stealth, or megastructures.

Technology branches include:

- Military
- Economy and production
- Logistics
- Sensors and intelligence
- Stealth and concealment
- Energy
- Population and medicine
- Education and innovation
- Automation and computing
- Space and exploration
- Epic Project research

Technology discovery has multiple forms:

- Predictable unlocks from known requirements
- Probabilistic breakthroughs from innovation score
- Discoveries from rare resources or environments
- Discoveries from specialists and buildings
- Discoveries from scouting, ruins, anomalies, or other civilizations
- Round-specific discoveries connected to Epic Projects

Some technologies are known goals. Others are hidden until the civilization
discovers the right conditions. A player may know that better metallurgy exists,
but not know that a specific anomaly can lead to portal physics.

Innovation increases the chance and speed of discovering eligible technologies,
but it does not guarantee that every civilization discovers the same things in
the same order.

Technology creates asymmetric advantage. A civilization with a smaller army may
become more dangerous through better weapons, range, automation, energy,
sensors, stealth, medicine, or logistics. A civilization with a large early army
may dominate nearby enemies but fall behind if it neglects education,
specialists, infrastructure, and innovation.

Exact enemy technology is usually unknown. Players infer it from reports and
events:

- Enemy units move faster than expected.
- A scout sees unfamiliar building shapes.
- A battle report shows unusual weapon effects.
- A settlement emits radio signals.
- A missile launch is detected.
- A hidden colony is discovered.
- A sensor report detects abnormal energy use.
- An Epic Project anomaly appears.

This means technological progress is both power and information warfare. Players
hide what they know, reveal only what they must, and make strategic decisions
from incomplete evidence.

## Starting Conditions and Early Progression

Each player starts as a small primitive civilization. The starting civilization
has only basic survival knowledge, local map awareness, and a small population.

The start represents a stone-age or early tribal stage:

- No metalworking
- No writing
- No formal science
- No long-distance communication
- No advanced storage
- No large-scale agriculture
- No known enemies
- No knowledge of the wider world

The player starts with one small settlement area. The settlement has a few basic
people, a small amount of food, simple materials, and local terrain knowledge.
Everything beyond the immediate area is unknown until explored.

Initial civilization state can include:

- Small population
- Low carrying capacity
- Basic happiness
- Basic health
- No assigned specialists
- No trained military
- No discovered advanced technologies
- Very low innovation score
- Only local map visibility

Initial resources are primitive and local:

- Food
- Wood
- Stone
- Basic hides, fibers, or similar natural materials

Metal resources can exist on the map, but the player does not understand how to
extract or use them until the right discoveries are made.

Initial buildings are basic survival structures:

- Campfire or central camp
- Shelter or hut
- Food gathering site
- Woodcutting site
- Storage pit
- Simple workshop
- Scout camp

Initial units are ordinary people with simple roles:

- Villager
- Gatherer
- Builder
- Scout
- Primitive warrior

Early gameplay focuses on survival, growth, and local exploration:

```text
Gather food and wood.
-> Build shelter and storage.
-> Increase population cap.
-> Send scouts.
-> Discover nearby resources.
-> Improve food security.
-> Create first specialized buildings.
-> Increase innovation.
-> Discover agriculture, writing, pottery, mining, or metalworking.
```

The first strategic choice is how to use limited population:

- More gatherers improve survival and resource income.
- More builders speed up construction.
- More scouts reveal resources and threats.
- More primitive warriors improve safety but slow growth.
- More teachers or early knowledge roles improve innovation once unlocked.

Primitive military exists, but early army building has a high opportunity cost.
A player can train warriors for defense or raids, but those people stop
contributing to gathering, building, scouting, and early innovation.

Early progression is driven by both construction and discovery. Buildings create
conditions for new discoveries:

- Repeated food gathering can lead to agriculture.
- Storage pits can lead to pottery or preservation.
- Simple workshops can lead to tools.
- Scout reports can reveal stone, ore, animals, rivers, or ruins.
- Population growth can create demand for better housing and organization.
- Early teaching or ritual sites can lead to writing, schools, or formal
  knowledge.

The first major transition is from primitive survival to organized settlement.
This transition unlocks better food production, more stable population growth,
larger buildings, early specialists, and the first meaningful innovation engine.

The second major transition is access to metals. Metalworking changes the
civilization's military, tools, construction, storage, and production capacity.
This is the first moment where civilizations can start diverging sharply based
on local resources, discoveries, and strategic choices.

## MVP Pages

The first playable version focuses on the core loop: persistent civilization
growth, map awareness, buildings, resources, population, units, technology, and
reports. Concrete MVP behavior is documented in
[user-stories](user-stories/README.md).

Each page answers a clear player question:

```text
Overview: What is happening?
Map: What do I know about the world?
Buildings: What am I constructing?
Resources: What can I afford?
Population: What are my people doing?
Units: What forces do I control?
Technology: What can my civilization become?
Mailbox: What did my civilization learn?
```

### Overview

The Overview page summarizes the current state of the civilization.

It shows:

- Population
- Happiness
- Innovation score
- Resource stockpiles
- Resource production and upkeep
- Active warnings
- Current construction progress
- Current unit training progress
- Current technology discoveries or breakthrough chances
- Recent mailbox reports

### Map

The Map page shows terrain, settlements, buildings, units, movement, and fog of
war.

It shows:

- Known terrain
- Unknown, explored, observed, and controlled areas
- Owned buildings
- Sensor coverage
- Scouts and armies
- Movement paths
- Known enemy locations
- Estimated enemy activity
- Report markers and anomalies

### Buildings

The Buildings page manages construction and upgrades.

It shows:

- Available buildings
- Building requirements
- Construction costs
- Construction time
- Building queue
- Upgrade options
- Production effects
- Storage effects
- Sensor effects
- Population cap effects
- Innovation effects

### Resources

The Resources page explains what the civilization produces, stores, and
consumes.

It shows:

- Current stockpiles
- Production rates
- Storage capacity
- Consumption
- Unit upkeep
- Building upkeep
- Resource shortages
- Resource overflow warnings

### Population

The Population page explains the human capacity of the civilization.

It shows:

- Total population
- Available population
- Assigned population
- Soldiers
- Population cap or carrying capacity
- Happiness
- Health
- Growth rate
- Growth modifiers
- Role assignments

### Units

The Units page manages trained units, scouts, armies, and assigned population.

It shows:

- Available unit types
- Unit requirements
- Unit training costs
- Training time
- Training queue
- Existing units
- Assigned population
- Unit upkeep
- Unit location
- Movement, scouting, attack, and defense actions

### Technology

The Technology page shows known discoveries and possible development paths.

It shows:

- Current technology stage
- Known technologies
- Available research directions
- Innovation score
- Discovery requirements
- Breakthrough chances
- Recent discoveries
- Unknown or partially discovered technology hints

### Mailbox

The Mailbox page contains the reports, alerts, discoveries, intelligence, and
messages known to the civilization.

It shows:

- Building reports
- Unit reports
- Scout reports
- Research discoveries
- Resource warnings
- Attack warnings
- Sensor reports
- Strategic anomaly reports
- Player messages
- Report source
- Report confidence
- Last observed time
- Action links

## Unit

Unit construction takes some amount of resources and population. After construction these units have certain maintanence cost. Units are movable and can move by themself. One unit can be for example:

- One kokobongo with glub. Which only require one population per unit.
- Something like [Contubernium](<https://en.wikipedia.org/wiki/Contubernium_(Roman_army_unit)>) which has few soldiers and requires something around 8 population.
- Tanks which might require around 4 population.
- Battleship which might require 100 to 1000 population.
- Big spachips tink of something like in starwars where they might be thousands to hundereds of thousands.

Units have basic state and then they can be trained for more advanced roles like engineer or doctor. Upgrades to different units have various requirements like simple rifle squad require units with basic military training but something like battleship might require chefs, engineers, doctors... Some upgrades to units have some requirements like certain age or IQ level.

Units can have multiple upgrades but obviously it is costly and might not make sense. Upgrades like new education are not cancellable but things like riffle squat and be dismantled and units like tanks could give resources if dismantled but obviously less than was used to build them.

### Properties

- Type of unit
- Health of unit
- Accuracy
- Attack damage
- Location either coordinates or building

## Battles

Even though game speed is speed up considerably speed up time is relevant in the battles for example units have different attack ranges which means certain type of unit can attack other type units farther away. Units will respond automatically to attack by attacking it if they can see it. Basically it measn that with acher attacks swordman then it would be logical that swordman will run to the archer but some missile is launched from 50km or more away how could some simple riffle squad know how to attack enemy which they cant see.

Battles are always onesided meaning that they are simulated from point of view of the attacker but both players are attackers if units are in attack range. Battle is cosidered to be started only when distance is less or equal than attack distance.

### Damage

Things that affect damage done to the enemy unit are:

- Attack damage
- Accuracy
- Enemy unit defense stat which is different to different damage types
- Attack speed

## Currency

There is no single currency players can create their own or choose to use existing ones. Value of the currency is determined by the player based on how much there is it and population and exchange rates.

Factories can use money to give assign value for the resource they are producing. This allows unifactories to efficiently obtain resources for factories.

## Map

One grid rectangle is 10 meters

One mapgrid size could be 10 000 km2 so around 1 000 000 rectangles.

With unit speed of 8 kmh it would take around 1400 hours to go through the whole map.

Map can have different types of areas like sea or areas which have high natural resources like oil or other minerals.

One map grid presents one planet and there can be map grids with different sizes. Players can only leave their map grid when they develop required technology to do so.

### Space

Maybe space could be three dimensional map grid where structures need to be placed on three dimensions

## Speed

It would be fun if the game time would have realistic time but it would be just speedup by x amount. However in real life difference between movent speed and building speed is quite high. For example if school building 8 month and the we would speed up time by 1000 then building this building would take few hours and going through the whole map would take 1 hour.

## Building

Building buildings require resources and workers. Also requires certain Technology stage. Buildings may also require certain vehicles for building like building modern big building without any machines would be very slow.

Buidings have some dependecies like:

- Resources like stone, metal....
- Specialiced unit like teachers, engineers..
- Innovation level
- Building units

Buildings can be upgraded but then thet might require more specialiced units, recources and higher maintanence cost.

### Properties

- Type of building
- Owner of building
- Health of building
- Map and coordinates of building (how to handle space ?)
- Shape of building. Basically it can have have any shape possible to make with map grid rectangles.
- How much resources it produces.
- How much resources it can store.
- How many resources it stores.

### Resource production

Buildings can have resource production rates for different resources. For example:

- Farm can produce food
- Mine can produce metal
- Oil rig can produce oil
- Wood cutting place can produce wood

If building does not have enough storage left then it will stop producing resources. This is why it is important to have supply chains which move resources to long term storage or to other buildings or construction sites.

### Resource storage

Buildings can have storage for resources. For example:

- Farm can have storage for food but only limited amount not for long term storage.
- Warehouse can have storage for all resources and can be upgraded to have more storage. Used for long term storage.
- Mine can have storage for metal but only limited amount not for long term storage.

### Unit storage

Buildings can store units like normal houses can store people and military base can store soldiers. Also units like tanks require more specialiced storage. Buildings have flags for each type of units it can store.
