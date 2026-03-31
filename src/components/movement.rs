use bevy::prelude::*;

/// Core movement parameters for a character.
#[derive(Component, Debug, Clone)]
pub struct Movement {
    pub speed: f32,
    pub jump_force: f32,
    pub dash_speed: f32,
    /// How long after leaving a platform edge a jump is still allowed (seconds).
    pub coyote_time: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 200.0,
            jump_force: 450.0,
            dash_speed: 600.0,
            coyote_time: 0.1,
        }
    }
}

/// Whether the character is currently grounded.
#[derive(Component, Default)]
pub struct Grounded(pub bool);

/// Tracks coyote-time window.
#[derive(Component, Default)]
pub struct CoyoteTimer {
    pub elapsed: f32,
}

impl CoyoteTimer {
    /// Whether the coyote-time window is still open.
    pub fn can_jump(&self, coyote_time: f32) -> bool {
        self.elapsed <= coyote_time
    }
}

/// Facing direction for animation and ability targeting.
#[derive(Component, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Facing {
    #[default]
    Right,
    Left,
}

/// Applied during a Loko dash.
#[derive(Component)]
pub struct Dashing {
    pub duration: f32,
    pub elapsed: f32,
    pub direction: f32, // -1.0 or 1.0
}

impl Dashing {
    pub fn new(direction: f32, duration: f32) -> Self {
        Self { duration, elapsed: 0.0, direction }
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_default_values() {
        let m = Movement::default();
        assert!((m.speed - 200.0).abs() < f32::EPSILON);
        assert!((m.jump_force - 450.0).abs() < f32::EPSILON);
        assert!((m.dash_speed - 600.0).abs() < f32::EPSILON);
        assert!((m.coyote_time - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn grounded_default_is_false() {
        let g = Grounded::default();
        assert!(!g.0);
    }

    #[test]
    fn coyote_timer_default_can_jump() {
        let t = CoyoteTimer::default();
        assert!(t.can_jump(0.1));
    }

    #[test]
    fn coyote_timer_expired_cannot_jump() {
        let t = CoyoteTimer { elapsed: 0.5 };
        assert!(!t.can_jump(0.3));
    }

    #[test]
    fn coyote_timer_exact_boundary() {
        let t = CoyoteTimer { elapsed: 0.3 };
        assert!(t.can_jump(0.3)); // at boundary = can still jump
    }

    #[test]
    fn facing_default_is_right() {
        assert_eq!(Facing::default(), Facing::Right);
    }

    #[test]
    fn dashing_new_starts_at_zero() {
        let d = Dashing::new(1.0, 0.3);
        assert_eq!(d.elapsed, 0.0);
        assert!(!d.is_finished());
    }

    #[test]
    fn dashing_finished_when_elapsed_exceeds_duration() {
        let d = Dashing { duration: 0.3, elapsed: 0.5, direction: 1.0 };
        assert!(d.is_finished());
    }

    #[test]
    fn dashing_not_finished_during() {
        let d = Dashing { duration: 0.3, elapsed: 0.1, direction: -1.0 };
        assert!(!d.is_finished());
    }
}
