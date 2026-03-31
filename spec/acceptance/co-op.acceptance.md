# Acceptance: Local Co-Op

## Scenario: P2 joins without pausing

**Given** a level is active
**When** Enter is pressed (or gamepad Start)
**Then** the companion AI is disabled
**And** the companion entity is now controlled by P2
**And** gameplay is never interrupted

## Scenario: P2 leaves, AI resumes

**Given** P2 is active
**When** P2 presses Start again
**Then** the Player2 component is removed from companion
**And** CompanionAI is re-added
**And** companion resumes following P1 within 0.5s

## Scenario: Companion takeover in Toddler mode

**Given** Toddler difficulty
**And** P2 joins
**Then** P2 has simplified controls (3-button)
**And** P2 companion entity is invulnerable
**And** P2 companion deals 50% damage

## Scenario: Team-up zone escalation

**Given** a level with a team-up zone at (900-1100, 0-400)
**And** P2 is active
**When** both players are inside the zone
**Then** a CoOpTierChangedEvent is fired with to=TeamUp
**And** TeamUpStartedEvent fires
**And** both players are at full power

## Scenario: Boss co-op health scaling (Kid mode)

**Given** Kid difficulty
**And** P2 joins before or during a boss fight
**Then** boss max_health is multiplied by 1.5

## Scenario: Mid-fusion P2 join

**Given** fusion is active (FusionState::Fused)
**When** P2 joins
**Then** P1 controls DoubleTrouble movement
**And** P2 controls DoubleTrouble attacks
