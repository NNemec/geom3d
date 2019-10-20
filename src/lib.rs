#![feature(uniform_paths)]

 #![feature(core_intrinsics)] // std::core::instrinsics::assume

pub use vector::{VectorXYZ0, Vector0XYZ, VectorXYZW, VectorWXYZ};

mod vector;
