#[cfg(test)]
mod tests {
    use shape_calculator::shapes::{circle::Circle, rectangle::Rectangle, square::Square};

    #[test]
    fn test_create_circle() {
        let circle = Circle::new(5.0);
        circle.print();
        assert_eq!(circle.rad, 5.0);
        assert_eq!(circle.area, 3.14 * 5.0 * 5.0);
    }

    #[test]
    fn test_create_rectangle() {
        let rectangle = Rectangle::new(3.0, 4.0);
        rectangle.print();
        assert_eq!(rectangle.w, 3.0);
        assert_eq!(rectangle.l, 4.0);
        assert_eq!(rectangle.area, 3.0 * 4.0);
    }

    #[test]
    fn test_create_square() {
        let square = Square::new(5.0);
        square.print();
        assert_eq!(square.dim, 5.0);
        assert_eq!(square.area, 5.0 * 5.0);
    }
}
