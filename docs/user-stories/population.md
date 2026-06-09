# Population

## View Population

As a player, I can view population state so that I understand my civilization's
human capacity.

Acceptance details:

- The population page shows total population.
- The population page shows available population.
- The population page shows assigned population by role.
- The population page shows carrying capacity.
- The population page shows happiness, health, and growth rate.

## Grow Population

As a player, my population grows or declines over persistent time so that food,
housing, health, and happiness matter.

Acceptance details:

- Growth uses elapsed real-world time.
- Growth slows as population approaches carrying capacity.
- Food shortage, low health, or low happiness reduces growth.
- Severe shortage can cause population decline.
- Population changes create reports when they are significant.

## Assign Population To Role

As a player, I can assign available population to roles so that I can choose
between growth, production, defense, exploration, and innovation.

Acceptance details:

- Assigning population reduces available population.
- Assigned roles affect production, training, scouting, or innovation.
- Soldiers are assigned population, not a normal resource purchase.
- Dead units remove their assigned population.
- Disbanded units can return some population to civilian life.
