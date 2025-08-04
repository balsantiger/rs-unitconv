/// cm → inch (1 inch = 2.54 cm)
pub fn cm_to_inch(cm: f64) -> f64 {
    cm / 2.54
}

/// inch → cm
pub fn inch_to_cm(inch: f64) -> f64 {
    inch * 2.54
}

pub fn mile_to_cm(mile: i32) -> i32 {
    mile * 160_900
}

pub fn mile_to_inch(mile: i32) -> i32 {
    mile * 63_360
}