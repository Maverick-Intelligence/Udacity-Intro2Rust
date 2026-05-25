use super::shape::area;

#[derive(Debug)]
pub struct Square {
    t: u8,
    pub dim: f32,
    pub area: f32,
}

impl Square {
    pub fn new(length_or_width: f32) -> Self {
        let t = 3;
        Self {
            t,
            dim: length_or_width,
            area: area(t, length_or_width),
        }
    }

    pub fn print(&self) {
        let st = if self.t == 3 {
            String::from("Square")
        } else {
            String::from("No Type")
        };
        println!("The {} has an area of {}m2", st, self.area);
    }
}
