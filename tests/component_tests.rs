use rust_native::{
    components::{Button, Component},
    geometry::Point,
};

#[test]
fn test_button_bounds() {
    let button = Button::new("Test Button");
    let bounds = button.bounds();
    assert_eq!(bounds.size.width, 100.0);
    assert_eq!(bounds.size.height, 40.0);
}

#[test]
fn test_button_contains_point() {
    let button = Button::new("Test Button");
    let point_inside = Point::new(50.0, 20.0);
    let point_outside = Point::new(150.0, 150.0);
    
    assert!(button.contains(point_inside));
    assert!(!button.contains(point_outside));
}
