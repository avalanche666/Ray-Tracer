use crate::{Custom_Point, dot, Ray};
use crate::hittable::Hittable;

struct Sphere {
    center :Custom_Point,
    radius :f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::Hit_Record) -> bool {
        let po :Custom_Point = r.origin() - center; // O-P
        let a :f64 = dot(r.direction(), r.direction()); // DD
        let b :f64 = 2.0 * dot(po, r.direction()); // 2DO - 2PD = 2(DO - PD) = 2D(O-P)
        let c :f64 = dot(po, po) - radius * radius; // -2OP + OO + PP - r*r = (O-P)^2 - r^2

        let discriminant :f64 = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return false;
        }

        let mut result :f64 = (-b - discriminant.sqrt())/(2.0*a);
        if result < t_min || t_max < result {
            result = (-b + discriminant.sqrt())/(2.0*a);
            if result < t_min || t_max < result {
                return false;
            }
        }

        rec.t = result;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - center) / radius;

        true
    }
}