use std::ops::{self, Add, AddAssign, Mul, MulAssign};

/// A vec3.
#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.data[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.data[1]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.data[2]
    }

    pub fn dot(&self, other: &Self) -> f32 {
        return self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x * y)
            .sum();
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - other.y() * self.z(),
            self.z() * other.x() - other.z() * self.x(),
            self.x() * other.y() - other.x() * self.y(),
        )
    }

    pub fn len(&self) -> f32 {
        return self.len_squared().sqrt();
    }

    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn normalize(&self) -> Self {
        *self / self.len()
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl ops::AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            data: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return self.add(-rhs);
    }
}

impl ops::SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(-rhs);
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            data: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            data: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        return self.mul(rhs.recip());
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.mul_assign(rhs.recip());
    }
}

/// Represents a color.
pub type Color = Vec3;

/// Represents a point in 3d space
pub type Point3 = Vec3;

impl Color {
    pub fn write_ppm<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        write!(
            writer,
            "{} {} {}\n",
            (self.x() * 255.999) as u8,
            (self.y() * 255.999) as u8,
            (self.z() * 255.999) as u8,
        )
    }
}
