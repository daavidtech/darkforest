# Buildings

## Place Building

As a player, I can place an available building on the map so that my
civilization gains new capabilities.

Acceptance details:

- The player can choose from currently available building types.
- The map previews the building shape before placement.
- Placement is blocked if the shape overlaps occupied or invalid tiles.
- Placement consumes required resources when construction starts.
- A construction queue item is created with a real-world finish time.

## Complete Building Construction

As a player, I can have buildings complete over persistent time so that my
civilization progresses while time passes.

Acceptance details:

- Construction progress is based on server time.
- A completed building becomes active.
- Active buildings apply their production, storage, sensor, population, or
  innovation effects.
- A building completion report is added to the mailbox.

## Upgrade Building

As a player, I can upgrade a building so that an existing location becomes more
valuable.

Acceptance details:

- The player can view upgrade requirements.
- Upgrades consume resources and take real-world time.
- Upgraded buildings improve at least one effect.
- Upgrade completion creates a mailbox report.
