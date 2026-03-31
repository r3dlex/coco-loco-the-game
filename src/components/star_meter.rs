use bevy::prelude::*;

/// Shared star meter — both P1 and P2 collect into this single resource.
///
/// Lives on the world (as a resource in `resources::StarMeter`, not a component),
/// but individual stars collected by characters are tracked via this component
/// on each spawned star entity for physics/despawn purposes.
#[derive(Component)]
pub struct StarPickup {
    /// Which player hit this star last (for re-collection after scatter).
    pub last_touch: Option<Entity>,
}

/// Marker: this star was scattered from a hit and is bouncing to its resting spot.
#[derive(Component)]
pub struct ScatteredStar {
    pub velocity: Vec2,
    pub elapsed: f32,
}
