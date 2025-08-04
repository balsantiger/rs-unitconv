use unitconv::length::*;
use unitconv::temperature::*;
use unitconv::weight::*;

#[test]
fn test_inch_to_cm() {
    assert_eq!(inch_to_cm(1.0), 2.54);
}

#[test]
fn test_kg_to_lb() {
    let lb = kg_to_lb(1.0);
    assert!((lb - 2.20462).abs() < 0.0001);
}

#[test]
fn test_mile_to_inch() {
    let inch = mile_to_inch(15);
    assert_eq!(inch, 950400);
}

#[test]
fn test_to_lb() {
    let lb = kg_to_lb(1.0);
    assert!((lb - 2.20462).abs() < 0.0001);
}

#[test]
fn test_f_to_c() {
    let c = f_to_c(32.0); // 32°F == 0°C
    assert!((c - 0.0).abs() < 0.0001);

    let c = f_to_c(212.0); // 212°F == 100°C
    assert!((c - 100.0).abs() < 0.0001);

    let c = f_to_c(-40.0); // -40°F == -40°C (동일)
    assert!((c + 40.0).abs() < 0.0001);
}