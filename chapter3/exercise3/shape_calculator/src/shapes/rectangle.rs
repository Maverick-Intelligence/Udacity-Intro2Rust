use super::shape::area;

#[derive(Debug)]
pub struct Rectangle {
    t: u8,
    pub w: f32,
    pub l: f32,
    pub area: f32,
}

impl Rectangle {
    pub fn new(width: f32, length: f32) -> Self {
        let t = 2;
        Self {
            t,
            w: width,
            l: length,
            area: area(t, (width, length)),
        }
    }

    pub fn print(&self) {
        let st = if self.t == 2 {
            String::from("Rectangle")
        } else {
            String::from("No Type")
        };
        println!("The {} has an area of {}m2", st, self.area);
    }
}
