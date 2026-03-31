use coco_loco::resources::star_meter::StarMeter;

/// Integration: star meter full-cycle across difficulty modes.
#[test]
fn toddler_meter_fills_at_5_stars() {
    let mut m = StarMeter::new(5);
    for i in 0..4 {
        assert!(!m.add(1), "star {i} should not fill the meter");
    }
    assert!(m.add(1), "star 5 should fill the meter");
    assert!(m.is_full());
}

#[test]
fn kid_meter_fills_at_10_stars() {
    let mut m = StarMeter::new(10);
    for i in 0..9 {
        assert!(!m.add(1), "star {i} should not fill the meter");
    }
    assert!(m.add(1), "star 10 should fill the meter");
    assert!(m.is_full());
}

#[test]
fn scatter_and_refill_cycle() {
    let mut m = StarMeter::new(5);

    // Fill
    m.add(5);
    assert!(m.is_full());

    // Hit: lose 1 star (Toddler)
    let lost = m.scatter(1);
    assert_eq!(lost, 1);
    assert_eq!(m.current, 4);
    assert!(!m.is_full());

    // Re-collect the scattered star
    assert!(m.add(1));
    assert!(m.is_full());
}

#[test]
fn hit_scatter_kid_mode_3_stars() {
    let mut m = StarMeter::new(10);
    m.add(7);

    let lost = m.scatter(3);
    assert_eq!(lost, 3);
    assert_eq!(m.current, 4);
}

#[test]
fn multiple_hits_drain_meter() {
    let mut m = StarMeter::new(5);
    m.add(5);

    m.scatter(2);
    m.scatter(2);
    m.scatter(2); // only 1 left, should lose 1
    assert_eq!(m.current, 0);
}
