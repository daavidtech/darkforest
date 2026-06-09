# Scouting

## Send Scout To Unknown Tile

As a player, I can send a scout to an unknown tile so that I can learn what
exists beyond my settlement.

Acceptance details:

- The player can select an available scout.
- The player can choose an unknown or explored destination tile.
- The scout travels over real time.
- Travel time depends on distance, terrain, scout speed, and logistics.
- The scout is unavailable while traveling.
- The destination remains uncertain until the scouting result is received.

## Receive Scout Report

As a player, I can receive a scout report so that the map gains new knowledge
without revealing perfect truth.

Acceptance details:

- The report is added to the mailbox when the scout observes or returns.
- The target tile becomes explored or observed based on scout behavior.
- The report includes terrain.
- The report includes visible resources.
- The report includes visible buildings or units if detected.
- The report includes confidence and observation time.
- Enemy information from the report decays over time if not observed again.
