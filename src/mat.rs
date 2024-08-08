use super::vec::Dot;
use super::{Vec3, Vec4};

/// Matrices that can be transposed
pub trait Transpose {
    /// Transposes a matrix, turning rows into columns and vice-versa
    fn transpose(self) -> Self;
}

/// A 4x4 matrix in column-major order
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
    c0: Vec4,
    c1: Vec4,
    c2: Vec4,
    c3: Vec4,
}

impl Transpose for Mat4 {
    fn transpose(self) -> Self {
        Self {
            c0: Vec4::new(self.c0.x, self.c1.x, self.c2.x, self.c3.x),
            c1: Vec4::new(self.c0.y, self.c1.y, self.c2.y, self.c3.y),
            c2: Vec4::new(self.c0.z, self.c1.z, self.c2.z, self.c3.z),
            c3: Vec4::new(self.c0.w, self.c1.w, self.c2.w, self.c3.w),
        }
    }
}

impl std::ops::Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.c0,
            1 => &self.c1,
            2 => &self.c2,
            3 => &self.c3,
            _ => panic!("Invalid column index into Mat4"),
        }
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut cols: [Vec4; 4] = [num::zero(); 4];
        let rhs = rhs.transpose();
        for i in 0..4 {
            let mut v: [f32; 4] = [0.0; 4];
            for j in 0..4 {
                v[j] = self[i].dot_mul(rhs[j]);
            }
            cols[i] = Vec4::new(v[0], v[1], v[2], v[3]);
        }

        Self {
            c0: cols[0],
            c1: cols[1],
            c2: cols[2],
            c3: cols[3],
        }
    }
}

impl std::ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            c0: self.c0 + rhs.c0,
            c1: self.c1 + rhs.c1,
            c2: self.c2 + rhs.c2,
            c3: self.c3 + rhs.c3,
        }
    }
}

impl num::One for Mat4 {
    fn one() -> Self {
        Self {
            c0: Vec4::new(1.0, 0.0, 0.0, 0.0),
            c1: Vec4::new(0.0, 1.0, 0.0, 0.0),
            c2: Vec4::new(0.0, 0.0, 1.0, 0.0),
            c3: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

/// Creates a new matrix corresponding to the supplied matrix composed with a translate operation
pub fn translate(mat: &Mat4, vec: Vec3) -> Mat4 {
    Mat4 {
        c0: Vec4::new(mat.c0.x, mat.c1.x, mat.c2.x, mat.c3.x + vec.x),
        c1: Vec4::new(mat.c0.y, mat.c1.y, mat.c2.y, mat.c3.y + vec.y),
        c2: Vec4::new(mat.c0.z, mat.c1.z, mat.c2.z, mat.c3.z + vec.z),
        c3: Vec4::new(mat.c0.w, mat.c1.w, mat.c2.w, mat.c3.w),
    }
}

impl crate::AsArray for Mat4 {
    type Output = f32;

    fn as_array(&self) -> impl AsRef<[Self::Output]> {
        self.c0
            .as_array()
            .as_ref()
            .iter()
            .chain(self.c1.as_array().as_ref().iter())
            .chain(self.c2.as_array().as_ref().iter())
            .chain(self.c3.as_array().as_ref().iter())
            .copied()
            .collect::<Vec<_>>()
    }
}
