use shape_calculator::shapes::{circle::Circle, rectangle::Rectangle, square::Square};

fn main() {
    let circle = Circle::new(1.0);
    circle.print();

    let rect = Rectangle::new(3.0, 4.0);
    rect.print();

    let square = Square::new(5.0);
    square.print();
}
