use std::rc::Rc;
use crate::hittable::{Hit_Record, Hittable};
use crate::Ray;

struct Hittable_List {
    objects :Vec<Hittable>
}

impl Hittable_List {
    pub fn clear (&mut self) {
        self.objects.clear();
    }

    pub fn add (&mut self, object :Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for Hittable_List {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::Hit_Record) -> bool {
        let mut temp :Hit_Record;
        let mut hit_scan :bool = false;
        let mut far = t_max;

        for element in self.objects.iter() {
            if self.objects[element] {
                hit_scan = true;
                far = temp.t;
                rec = temp;
            }
        }

        hit_scan
    }
}