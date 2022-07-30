pub const INFINITY :f64 = f64::INFINITY;
pub const PI :f64 = 3.1415926535897932385;

pub fn degree_to_radians (degree :f64) -> f64 {
    degree * PI / 180.0
}
