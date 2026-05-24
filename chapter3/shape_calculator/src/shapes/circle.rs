use super::shape::area;

#[derive(Debug)]
pub struct Circle {
    t: u8,
    pub rad: f32,
    pub area: f32,
}

impl Circle {
    pub fn new(rad: f32) -> Self {
        let t = 1;
        Self {
            t,
            rad,
            area: area(t, rad),
        }
    }

    pub fn print(&self) {
        let st = if self.t == 1 {
            String::from("Circle")
        } else {
            String::from("No Type")
        };
        println!("The {} has an area of {}m2", st, self.area);
    }
}
