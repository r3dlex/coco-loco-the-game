# Acceptance: Double Trouble

## Scenario: Toddler auto-fusion on full meter

**Given** Toddler difficulty
**And** the star meter has 5 stars
**When** the 5th star is collected
**Then** FusionActivatedEvent fires immediately
**And** FusionState transitions to Activating then Fused

## Scenario: Kid manual fusion

**Given** Kid difficulty
**And** the star meter has 10 stars
**When** P1 presses F
**Then** FusionActivatedEvent fires
**And** FusionState transitions to Fused

## Scenario: Kid fusion does not auto-fire

**Given** Kid difficulty
**And** the star meter has 10 stars
**And** P1 does not press F
**Then** FusionActivatedEvent is NOT fired automatically

## Scenario: Fusion lasts 12 seconds

**Given** FusionState is Fused
**Then** after 12.0 seconds FusionExpiredEvent fires
**And** brothers pop apart (DoubleTrouble despawned)
**And** both character entities become Visible

## Scenario: Fusion flicker warning

**Given** FusionState is Fused
**And** elapsed >= 9.0 seconds (3s remaining)
**Then** entities with FusionFlicker component toggle Visibility rapidly

## Scenario: Toddler cooldown: meter refill

**Given** Toddler difficulty
**And** FusionExpiredEvent fires
**Then** FusionState returns to Normal (no Cooldown state entered)
**And** the meter is at 0

## Scenario: Kid 45-second cooldown

**Given** Kid difficulty
**And** FusionExpiredEvent fires
**Then** FusionState enters Cooldown
**And** after 45 seconds FusionState returns to Normal
