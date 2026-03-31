# Acceptance: Input Feedback

## Scenario: Key press produces visual and audio feedback

**Given** a scene is loaded with default feedback mappings
**When** any key is pressed
**Then** a visual effect appears on screen within 50ms
**And** a sound plays concurrently

## Scenario: Mouse click produces feedback at click position

**Given** a scene is loaded
**When** the user clicks at position (x, y)
**Then** a visual effect appears centred on (x, y)
**And** a sound plays

## Scenario: Multiple rapid inputs produce independent feedback

**Given** a scene is loaded
**When** 5 keys are pressed within 200ms
**Then** 5 independent visual effects appear
**And** 5 sounds play (overlapping is acceptable)

## Scenario: No scene-specific mapping falls back to default

**Given** a scene is loaded with no `on_key_press` handler
**When** a key is pressed
**Then** the global default feedback triggers
