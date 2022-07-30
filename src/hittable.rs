use crate::{Custom_Point, Custom_Vector, dot, Ray};

#[derive(Default)]
pub struct Hit_Record {
    pub p :Custom_Point,
    pub normal :Custom_Vector,
    pub t :f64,
    pub front_face :bool
}

impl default for Hit_Record {
    fn default() -> Self {
        Self {
            p: (Custom_Point::new(0.0, 0.0, 0.0)),
            normal: (Custom_Vector::new(0.0, 0.0, 0.0)),
            t: 0.0,
            front_face: false
        }
    }
}

impl Hit_Record {
    pub fn set_face_normal (&mut self, r :&Ray, outward_normal :Custom_Vector) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        if let true = self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit (&self, r :&Ray, t_min :f64, t_max :f64, rec :&mut Hit_Record) -> bool;
}