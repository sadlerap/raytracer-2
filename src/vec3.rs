use std::{
    f32::consts::TAU,
    iter::Sum,
    num::NonZeroU32,
    ops::{self, Add, AddAssign, Mul, MulAssign},
};

use rand::{random, thread_rng, Rng};

use crate::util::Range;

/// A vec3.
#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn random() -> Self {
        Self::new(rand::random(), rand::random(), rand::random())
    }

    pub fn random_in_range(min: f32, max: f32) -> Self {
        let x = rand::thread_rng().gen_range(min..max);
        let y = rand::thread_rng().gen_range(min..max);
        let z = rand::thread_rng().gen_range(min..max);
        Self::new(x, y, z)
    }

    pub fn random_on_unit_sphere() -> Self {
        // See https://mathworld.wolfram.com/SpherePointPicking.html for why this works.
        let theta = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
        let u: f32 = rand::thread_rng().gen_range(-1.0..1.0);

        let (sin_theta, cos_theta) = theta.sin_cos();
        let sin_phi = u.mul_add(-u, 1.0).sqrt();

        let x = cos_theta * sin_phi;
        let y = sin_theta * sin_phi;
        let z = u;

        Self::new(x, y, z)
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_on_unit_sphere();
        if on_unit_sphere.dot(normal).is_sign_positive() {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disc() -> Self {
        let r = random::<f32>().sqrt();
        let theta = thread_rng().gen_range(0.0..TAU);

        let x = r * theta.cos();
        let y = r * theta.sin();
        Self::new(x, y, 0.0)
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
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x * y)
            .sum()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - other.y() * self.z(),
            self.z() * other.x() - other.z() * self.x(),
            self.x() * other.y() - other.x() * self.y(),
        )
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn normalize(&self) -> Self {
        *self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        // is the vector close to zero?
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(&n) * n
    }

    pub fn refract(&self, n: Vec3, eta_i_over_eta_t: f32) -> Vec3 {
        let cos_theta = (-*self).dot(&n).min(1.0);
        let r_out_perp = eta_i_over_eta_t * (*self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;

        r_out_perp + r_out_parallel
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

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc + x).unwrap_or_default()
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
        self.add(-rhs)
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
        self.mul(rhs.recip())
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
    pub fn write_ppm<W: std::io::Write>(
        &self,
        writer: &mut W,
        samples_per_pixel: NonZeroU32,
    ) -> std::io::Result<()> {
        let scale = (u32::from(samples_per_pixel) as f32).recip();

        let r = linear_to_gamma(self.x() * scale);
        let g = linear_to_gamma(self.y() * scale);
        let b = linear_to_gamma(self.z() * scale);

        static INTENSITY: crate::util::Range<f32> = Range::new(0.0, 0.999);
        writeln!(
            writer,
            "{} {} {}",
            (256.0 * INTENSITY.clamp(r)) as u8,
            (256.0 * INTENSITY.clamp(g)) as u8,
            (256.0 * INTENSITY.clamp(b)) as u8,
        )
    }
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    linear_component.sqrt()
}
