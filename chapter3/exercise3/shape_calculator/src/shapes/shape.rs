pub trait AreaFor {
    fn area_for(&self, t: u8) -> f32;
}

impl AreaFor for f32 {
    fn area_for(&self, t: u8) -> f32 {
        let n = *self;
        match t {
            1 => 3.14 * n * n,
            3 => n * n,
            _ => 0.0,
        }
    }
}

impl AreaFor for (f32, f32) {
    fn area_for(&self, t: u8) -> f32 {
        match t {
            2 => self.0 * self.1,
            _ => 0.0,
        }
    }
}

pub fn area<T: AreaFor>(t: u8, num: T) -> f32 {
    num.area_for(t)
}
