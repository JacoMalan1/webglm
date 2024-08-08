#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::unwrap_used,
    missing_docs
)]

//! A WebGL math library
//!
//! # Features
//!
//! SIMD - `webglm` uses the [WASM SIMD Extension](https://github.com/WebAssembly/spec/blob/main/proposals/simd/SIMD.md) to
//! speed up computations.
//!
//! # Example
//!
//! The following code creates a 4x4 translation matrix
//! ```
//! let v = vec3(1.0, 2.0, 3.0);
//! let matrix = mat::translate(&num::one(), v);
//! ```

pub use vec::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

/// A trait for objects that can be turned into an array
pub trait AsArray {
    /// The element type of the generated array
    type Output;

    /// Turns an object into an array
    fn as_array(&self) -> impl AsRef<[Self::Output]>;
}

/// Matrices
pub mod mat;
/// Vectors
pub mod vec;
