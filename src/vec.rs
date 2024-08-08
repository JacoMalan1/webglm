use crate::AsArray;

/// Objects that have a well-defined magnitude (2-norm).
pub trait Magnitude {
    /// Computes the magnitude of `self`
    fn mag(&self) -> f32;
}

/// Marker trait to denote that an object is a Vector
pub trait Vector: AsArray + std::ops::Add + std::ops::Sub + Magnitude + Sized {}

/// A trait for things that can take a dot product with themselves
pub trait Dot {
    /// The result of the dot product
    type Output;

    /// Computes the dot product of `self` with `rhs`
    fn dot_mul(self, rhs: Self) -> Self::Output;
}

macro_rules! impl_vec_new {
    ($vec:ident, $($field:ident),+) => {
        impl $vec {
            /// Constructs a new `$vec`
            pub fn new($($field: f32),+) -> Self {
                Self {
                    $($field),+
                }
            }
        }
    }
}

macro_rules! impl_vec_zero {
    ($vec:ident, $($field:ident),+) => {
        impl ::num::Zero for $vec {
            fn zero() -> Self {
                Self {
                    $($field: 0.0),+
                }
            }

            fn is_zero(&self) -> bool {
                $(self.$field.is_zero())&&+
            }
        }
    };
}

macro_rules! impl_vec_array {
    ($vec:ident, $($field:ident),+) => {
        impl $crate::AsArray for $vec {
            type Output = f32;

            fn as_array(&self) -> impl AsRef<[Self::Output]> {
                [$(self.$field),+]
            }
        }
    }
}

macro_rules! impl_vec_mag {
    ($vec:ident, $($field:ident),+) => {
        impl Magnitude for $vec {
            fn mag(&self) -> f32 {
                self.dot_mul(*self).sqrt()
            }
        }
    };
}

/// A two-component vector of `f32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    /// The x component
    pub x: f32,
    /// The y component
    pub y: f32,
}

impl_vec_new!(Vec2, x, y);
impl_vec_zero!(Vec2, x, y);
impl_vec_array!(Vec2, x, y);
impl_vec_mag!(Vec2, x, y);

impl Vector for Vec2 {}

impl std::ops::Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
        }
    }
}

impl std::ops::Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
        }
    }
}

impl Dot for Vec2 {
    type Output = f32;

    fn dot_mul(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        std::arch::wasm32::f32x4_extract_lane::<0>(res)
            + std::arch::wasm32::f32x4_extract_lane::<1>(res)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, 0.0, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, 0.0, 0.0);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
        }
    }
}

/// A three-component vector of `f32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    /// The x (red) component
    pub x: f32,
    /// The y (green) component
    pub y: f32,
    /// The z (blue) component
    pub z: f32,
}

impl_vec_new!(Vec3, x, y, z);
impl_vec_zero!(Vec3, x, y, z);
impl_vec_array!(Vec3, x, y, z);
impl_vec_mag!(Vec3, x, y, z);

impl Vector for Vec3 {}

impl std::ops::Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, 0.0);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, 0.0);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, 0.0);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
        }
    }
}

impl Dot for Vec3 {
    type Output = f32;

    fn dot_mul(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, 0.0);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        std::arch::wasm32::f32x4_extract_lane::<0>(res)
            + std::arch::wasm32::f32x4_extract_lane::<1>(res)
            + std::arch::wasm32::f32x4_extract_lane::<2>(res)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, 0.0);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, 0.0);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, 0.0);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl num::One for Vec3 {
    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

/// A four-component vector of `f32`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4 {
    /// The x (red) component
    pub x: f32,
    /// The y (green) component
    pub y: f32,
    /// The z (blue) component
    pub z: f32,
    /// The w (alpha) component
    pub w: f32,
}

impl_vec_new!(Vec4, x, y, z, w);
impl_vec_zero!(Vec4, x, y, z, w);
impl_vec_array!(Vec4, x, y, z, w);
impl_vec_mag!(Vec4, x, y, z, w);

impl Vector for Vec4 {}

impl std::ops::Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, rhs);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
            w: std::arch::wasm32::f32x4_extract_lane::<3>(res),
        }
    }
}

impl std::ops::Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, rhs);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
            w: std::arch::wasm32::f32x4_extract_lane::<3>(res),
        }
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs, rhs, rhs, rhs);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
            w: std::arch::wasm32::f32x4_extract_lane::<3>(res),
        }
    }
}

impl Dot for Vec4 {
    type Output = f32;

    fn dot_mul(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, rhs.w);
        let res = std::arch::wasm32::f32x4_mul(s, rhs);

        std::arch::wasm32::f32x4_extract_lane::<0>(res)
            + std::arch::wasm32::f32x4_extract_lane::<1>(res)
            + std::arch::wasm32::f32x4_extract_lane::<2>(res)
            + std::arch::wasm32::f32x4_extract_lane::<3>(res)
    }
}

impl std::ops::Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, self.w);
        let res = std::arch::wasm32::f32x4_add(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
            w: std::arch::wasm32::f32x4_extract_lane::<3>(res),
        }
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let s = std::arch::wasm32::f32x4(self.x, self.y, self.z, self.w);
        let rhs = std::arch::wasm32::f32x4(rhs.x, rhs.y, rhs.z, self.w);
        let res = std::arch::wasm32::f32x4_sub(s, rhs);

        Self {
            x: std::arch::wasm32::f32x4_extract_lane::<0>(res),
            y: std::arch::wasm32::f32x4_extract_lane::<1>(res),
            z: std::arch::wasm32::f32x4_extract_lane::<2>(res),
            w: std::arch::wasm32::f32x4_extract_lane::<3>(res),
        }
    }
}

/// Computes the distance between two vectors using Pythagoras's theorem.
pub fn distance<V>(v1: &V, v2: &V) -> f32
where
    V: Vector + Copy,
    <V as std::ops::Sub>::Output: Vector,
{
    (*v2 - *v1).mag()
}

/// Creates a new two-component vector
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

/// Creates a new three-component vector
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

/// Creates a new four-component vector
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}
