use bevy::prelude::*;

/// Global, shared star meter. Both players contribute to the same meter.
#[derive(Resource)]
pub struct StarMeter {
    pub current: u32,
    pub max: u32,
}

impl StarMeter {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }

    /// Add stars and return whether the meter just became full.
    pub fn add(&mut self, count: u32) -> bool {
        let was_full = self.is_full();
        self.current = (self.current + count).min(self.max);
        !was_full && self.is_full()
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Scatter stars after a hit. Deducts from meter and returns count lost.
    pub fn scatter(&mut self, count: u32) -> u32 {
        let lost = count.min(self.current);
        self.current -= lost;
        lost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_zero() {
        let m = StarMeter::new(5);
        assert_eq!(m.current, 0);
        assert_eq!(m.max, 5);
    }

    #[test]
    fn add_increments_current() {
        let mut m = StarMeter::new(5);
        let became_full = m.add(2);
        assert!(!became_full);
        assert_eq!(m.current, 2);
    }

    #[test]
    fn add_returns_true_when_first_full() {
        let mut m = StarMeter::new(5);
        m.add(4);
        assert!(m.add(1));
    }

    #[test]
    fn add_returns_false_when_already_full() {
        let mut m = StarMeter::new(5);
        m.add(5);
        assert!(!m.add(1));
    }

    #[test]
    fn add_caps_at_max() {
        let mut m = StarMeter::new(5);
        m.add(10);
        assert_eq!(m.current, 5);
    }

    #[test]
    fn add_returns_true_on_exact_fill() {
        let mut m = StarMeter::new(3);
        assert!(m.add(3));
        assert!(m.is_full());
    }

    #[test]
    fn add_zero_does_not_fill() {
        let mut m = StarMeter::new(5);
        m.current = 5;
        assert!(!m.add(0));
    }

    #[test]
    fn is_full_when_at_max() {
        let mut m = StarMeter::new(5);
        assert!(!m.is_full());
        m.current = 5;
        assert!(m.is_full());
    }

    #[test]
    fn is_full_when_above_max() {
        let mut m = StarMeter::new(5);
        m.current = 10; // shouldn't happen, but defensive
        assert!(m.is_full());
    }

    #[test]
    fn reset_sets_to_zero() {
        let mut m = StarMeter::new(5);
        m.add(3);
        m.reset();
        assert_eq!(m.current, 0);
        assert!(!m.is_full());
    }

    #[test]
    fn scatter_reduces_current() {
        let mut m = StarMeter::new(10);
        m.add(6);
        let lost = m.scatter(3);
        assert_eq!(lost, 3);
        assert_eq!(m.current, 3);
    }

    #[test]
    fn scatter_caps_at_current() {
        let mut m = StarMeter::new(10);
        m.add(2);
        let lost = m.scatter(10);
        assert_eq!(lost, 2);
        assert_eq!(m.current, 0);
    }

    #[test]
    fn scatter_from_empty() {
        let mut m = StarMeter::new(5);
        let lost = m.scatter(5);
        assert_eq!(lost, 0);
        assert_eq!(m.current, 0);
    }

    #[test]
    fn full_cycle_add_scatter_add() {
        let mut m = StarMeter::new(5);
        m.add(5);
        assert!(m.is_full());
        m.scatter(3);
        assert!(!m.is_full());
        assert_eq!(m.current, 2);
        assert!(m.add(3)); // fills again
        assert!(m.is_full());
    }

    #[test]
    fn meter_size_one() {
        let mut m = StarMeter::new(1);
        assert!(m.add(1));
        assert!(m.is_full());
        m.scatter(1);
        assert!(!m.is_full());
    }
}
