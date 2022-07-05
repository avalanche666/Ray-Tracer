use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Custom_Vector {
    e :[f64;3],
}

impl Custom_Vector {
    pub fn new_from_zero() -> Self {
        Self {
            e :[0.0;3]
        }
    }

    pub fn new (x :f64, y :f64, z :f64) -> Self {
        Self {
            e :[x, y, z]
        }
    }

    pub fn x (&self) -> f64 {
        self.e[0]
    }

    pub fn y (&self) -> f64 {
        self.e[1]
    }

    pub fn z (&self) -> f64 {
        self.e[2]
    }

    pub fn length (&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn normalize (&self) -> Custom_Vector {
        unit_vector(&self)
    }

}

impl Neg for Custom_Vector {
    type Output = Custom_Vector;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl MulAssign<f64> for Custom_Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Custom_Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0f64 / rhs;
    }
}

impl AddAssign<Custom_Vector> for Custom_Vector {
    fn add_assign(&mut self, rhs: Custom_Vector) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add<Custom_Vector> for Custom_Vector {
    type Output = Custom_Vector;

    fn add(self, rhs: Custom_Vector) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub<Custom_Vector> for Custom_Vector {
    type Output = Custom_Vector;

    fn sub(self, rhs: Custom_Vector) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - self.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul<Custom_Vector> for f64 {
    type Output = Custom_Vector;

    fn mul(self, rhs: Custom_Vector) -> Self::Output {
        Self::Output {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl Mul<f64> for Custom_Vector {
    type Output = Custom_Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Custom_Vector {
    type Output = Custom_Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0f64 / rhs)
    }
}

impl Index<usize> for Custom_Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

pub fn dot(u: Custom_Vector, v: Custom_Vector) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Custom_Vector, v: Custom_Vector) -> Custom_Vector {
    Custom_Vector::new(
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    )
}

pub fn unit_vector (v :&Custom_Vector) -> Custom_Vector {
    (*v) / v.length()
}

pub type Custom_Point = Custom_Vector;
pub type Custom_Color = Custom_Vector;