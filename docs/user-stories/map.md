# Map

## View Known Map

As a player, I can view the known map so that I can understand my surroundings.

Acceptance details:

- Unknown tiles are visually distinct from explored tiles.
- Explored tiles show last known terrain.
- Observed tiles show current visible activity.
- Controlled tiles show strong local awareness.
- Outdated enemy information shows last observed time and confidence.

## View Sensor Coverage

As a player, I can view building sensor coverage so that I understand what my
civilization can currently observe.

Acceptance details:

- Owned buildings contribute sensor coverage around their location.
- Specialized buildings such as watchtowers show larger or better coverage.
- Sensor coverage distinguishes observed areas from merely explored areas.
- Coverage updates when buildings are placed, upgraded, completed, or destroyed.

## Inspect Tile Knowledge

As a player, I can inspect a tile so that I can see what my civilization knows
about it.

Acceptance details:

- The tile panel shows terrain if known.
- The tile panel shows resources if known.
- The tile panel shows visible buildings or units if currently observed.
- The tile panel shows stale information with confidence if not currently
  observed.
- Unknown tiles do not reveal hidden truth.
