# Acceptance: Scene Management

## Scenario: Default scene loads on startup

**Given** the game is launched
**Then** the default scene loads and becomes interactive within 2 seconds

## Scenario: Parent cycles to next scene

**Given** a scene is active
**When** the right arrow key is pressed
**Then** a crossfade transition begins
**And** the next scene becomes active within 500ms

## Scenario: Parent cycles to previous scene

**Given** a scene is active and it is not the first scene
**When** the left arrow key is pressed
**Then** a crossfade transition begins
**And** the previous scene becomes active within 500ms

## Scenario: Scene wraps around

**Given** the last scene is active
**When** the right arrow key is pressed
**Then** the first scene loads (wrap-around)

## Scenario: Broken scene falls back to default

**Given** a scene's `scene.rhai` contains a syntax error
**When** the game attempts to load that scene
**Then** the default scene loads instead
**And** a warning is logged
