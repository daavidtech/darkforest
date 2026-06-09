# Account

## Log In

As a player, I can log in so that I can access my civilization.

Acceptance details:

- The player can enter a username and password.
- Invalid credentials show an error without logging the player in.
- Valid credentials create an authenticated session.
- After login, the player is taken to the overview page.
- Authenticated API requests identify the current player.

## View Current Civilization

As a player, I can see which civilization I am controlling so that I understand
which account state is active.

Acceptance details:

- The navigation or overview shows the current username or civilization name.
- Logging out clears the authenticated session.
- Protected pages redirect unauthenticated players to login.
