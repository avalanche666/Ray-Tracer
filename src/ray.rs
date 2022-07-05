use crate::{Custom_Vector, vector};
use crate::vector::Custom_Point;

pub struct Ray {
    origin :Custom_Point,
    direction :Custom_Vector,
}

impl Ray {
    pub fn new(origin: &Custom_Point, direction: &Custom_Vector) -> Self {
        Self {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin (&self) -> Custom_Point {self.origin}

    pub fn direction (&self) -> Custom_Vector {self.direction}

    pub fn at (&self, t :f64) -> Custom_Point {
        self.origin + self.direction * t
    }
}