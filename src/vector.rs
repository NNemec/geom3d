// Copyright 2019 Norbert Nemec
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::mem;

use num_traits::{Float, Zero};

/******************************************************************************/

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorXYZ0<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
    _w: F,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector0XYZ<F: Float> {
    _w: F,
    pub x: F,
    pub y: F,
    pub z: F,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorXYZW<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorWXYZ<F: Float> {
    pub w: F,
    pub x: F,
    pub y: F,
    pub z: F,
}

/******************************************************************************/

macro_rules! impl_vector3_from_xyz { ($V:ident) => {
    impl<F: Float> $V<F> {
        #[inline]
        pub fn from_xyz(x: F, y: F, z: F) -> $V<F> {
            $V { x: x, y: y, z: z, _w: F::zero() }
        }
    }
}}

impl_vector3_from_xyz!(Vector0XYZ);
impl_vector3_from_xyz!(VectorXYZ0);

/******************************************************************************/

macro_rules! impl_vector3w_from_xyzw { ($V:ident) => {
    impl<F: Float> $V<F> {
        #[inline]
        pub fn from_xyz(x: F, y: F, z: F) -> $V<F> {
            $V { x: x, y: y, z: z, w: F::zero() }
        }

        #[inline]
        pub fn from_xyzw(x: F, y: F, z: F, w: F) -> $V<F> {
            $V { x: x, y: y, z: z, w: w }
        }
    }
}}

impl_vector3w_from_xyzw!(VectorWXYZ);
impl_vector3w_from_xyzw!(VectorXYZW);

/******************************************************************************/

macro_rules! impl_fixed_array_conversions { ($V:ident) => {
    impl<F: Float> AsRef<[F; 4]> for $V<F> {
        #[inline]
        fn as_ref(&self) -> &[F; 4] {
            unsafe { mem::transmute(self) }
        }
    }

    impl<F: Float> AsMut<[F; 4]> for $V<F> {
        #[inline]
        fn as_mut(&mut self) -> &mut [F; 4] {
            unsafe { mem::transmute(self) }
        }
    }

    impl<F: Float> Into<[F; 4]> for $V<F> {
        #[inline]
        fn into(self) -> [F; 4] {
            *self.as_ref()
        }
    }

    // can't convert from reference to reference, since array may not be aligned

    impl<F: Float> From<[F; 4]> for $V<F> {
        #[inline]
        fn from(v: [F; 4]) -> $V<F> {
            unsafe {
                let mut res: $V<F> = std::mem::uninitialized();
                *res.as_mut() = v;
                res
            }
        }
    }
}}

impl_fixed_array_conversions!(Vector0XYZ);
impl_fixed_array_conversions!(VectorXYZ0);
impl_fixed_array_conversions!(VectorWXYZ);
impl_fixed_array_conversions!(VectorXYZW);

/******************************************************************************/

macro_rules! impl_vector3_conversions { ($dstV:ident <- $srcV:ident) => {
    impl<F: Float> From<$srcV<F>> for $dstV<F> {
        #[inline]
        fn from(v: $srcV<F>) -> $dstV<F> {
            $dstV::<F>::from_xyz(v.x, v.y, v.z)
        }
    }
}}

impl_vector3_conversions!(Vector0XYZ <- VectorXYZ0);
impl_vector3_conversions!(VectorXYZ0 <- Vector0XYZ);

/******************************************************************************/

macro_rules! impl_vector3w_conversions { ($srcV:ident <- $dstV:ident) => {
    impl<F: Float> From<$srcV<F>> for $dstV<F> {
        #[inline]
        fn from(v: $srcV<F>) -> $dstV<F> {
            $dstV::<F>::from_xyzw(v.x, v.y, v.z, v.w)
        }
    }
}}

impl_vector3w_conversions!(VectorWXYZ <- VectorXYZW);
impl_vector3w_conversions!(VectorXYZW <- VectorWXYZ);

/******************************************************************************/

macro_rules! impl_vector3_operators { ($V:ident) => {

    impl<F: Float> std::ops::Add for $V<F> {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            unsafe {
                std::intrinsics::assume(self._w == F::zero());
                std::intrinsics::assume(other._w == F::zero());
            }
            $V { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, _w: self._w + other._w }
        }
    }

    impl<F: Float> std::ops::Sub for $V<F> {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            unsafe {
                std::intrinsics::assume(self._w == F::zero());
                std::intrinsics::assume(other._w == F::zero());
            }
            $V { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, _w: self._w - other._w }
        }
    }

    impl<F: Float> std::ops::Mul<F> for $V<F> {
        type Output = Self;
        fn mul(self, scalar: F) -> Self {
            unsafe {
                std::intrinsics::assume(self._w == F::zero());
            }
            $V { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar, _w: self._w * scalar }
        }
    }

/*
    impl<F: Float> std::ops::Mul<$V<F>> for F {
        type Output = $V<F>;
        fn mul(self, vector: $V<F>) -> Output {
            unsafe {
                std::intrinsics::assume(vector._w == F::zero());
            }
            $V { x: self * vector.x, y: self * vector.y, z: self * vector.z, _w: self * vector._w }
        }
    }
*/

    impl<F: Float> Zero for $V<F> {
        fn zero() -> $V<F> {
            $V { x: F::zero(), y: F::zero(), z: F::zero(), _w: F::zero() }
        }
        fn is_zero(&self) -> bool {
            unsafe {
                std::intrinsics::assume(self._w == F::zero());
            }
            self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self._w.is_zero()
        }
    }

}}

impl_vector3_operators!(Vector0XYZ);
impl_vector3_operators!(VectorXYZ0);


/******************************************************************************/

#[test]
fn test_constructors() {
    type F = f32;
    let x: F = 1.;
    let y: F = 2.;
    let z: F = 3.;
    let w: F = 42.;
    let zero: F = 0.;

    {
        let v = Vector0XYZ::from_xyz(x,y,z);
        assert_eq!(v.x, x);
        assert_eq!(Into::<[F;4]>::into(v), [zero,x,y,z]);
    }
    {
        let v = VectorXYZ0::from_xyz(x,y,z);
        assert_eq!(v.x, x);
        assert_eq!(Into::<[F;4]>::into(v), [x,y,z,zero]);
    }
    {
        let v = VectorWXYZ::from_xyz(x,y,z);
        assert_eq!(v.x, x);
        assert_eq!(v.w, zero);
        assert_eq!(Into::<[F;4]>::into(v), [zero,x,y,z]);
    }
    {
        let v = VectorXYZW::from_xyz(x,y,z);
        assert_eq!(v.x, x);
        assert_eq!(v.w, zero);
        assert_eq!(Into::<[F;4]>::into(v), [x,y,z,zero]);
    }
    {
        let v = VectorWXYZ::from_xyzw(x,y,z,w);
        assert_eq!(v.x, x);
        assert_eq!(v.w, w);
        assert_eq!(Into::<[F;4]>::into(v), [w,x,y,z]);
    }
    {
        let v = VectorXYZW::from_xyzw(x,y,z,w);
        assert_eq!(v.x, x);
        assert_eq!(v.w, w);
        assert_eq!(Into::<[F;4]>::into(v), [x,y,z,w]);
    }
}

#[test]
fn test_conversions() {
    type F = f32;
    let x: F = 1.;
    let y: F = 2.;
    let z: F = 3.;
    let w: F = 42.;
    let zero: F = 0.;

    {
        let v1 = Vector0XYZ::from_xyz(x,y,z);
        assert_eq!(Into::<[F;4]>::into(v1), [zero,x,y,z]);

        let v2 = VectorXYZ0::from(v1);
        assert_eq!(Into::<[F;4]>::into(v2), [x,y,z,zero]);

        let v3 = Vector0XYZ::from(v1);
        assert_eq!(Into::<[F;4]>::into(v3), [zero,x,y,z]);
    }

    {
        let v1 = VectorWXYZ::from_xyz(x,y,z);
        assert_eq!(Into::<[F;4]>::into(v1), [zero,x,y,z]);

        let v2 = VectorXYZW::from(v1);
        assert_eq!(Into::<[F;4]>::into(v2), [x,y,z,zero]);

        let v3 = VectorWXYZ::from(v2);
        assert_eq!(Into::<[F;4]>::into(v3), [zero,x,y,z]);
    }

    {
        let v1 = VectorWXYZ::from_xyzw(x,y,z,w);
        assert_eq!(Into::<[F;4]>::into(v1), [w,x,y,z]);

        let v2 = VectorXYZW::from(v1);
        assert_eq!(Into::<[F;4]>::into(v2), [x,y,z,w]);

        let v3 = VectorWXYZ::from(v2);
        assert_eq!(Into::<[F;4]>::into(v3), [w,x,y,z]);
    }
}

#[test]
fn test_operators() {
    type F = f32;
    type V = Vector0XYZ<F>;

    {
        let v123 = V::from_xyz(1.,2.,3.);
        let v123neg= V::from_xyz(-1.,-2.,-3.);
        let v246 = V::from_xyz(2.,4.,6.);

        assert!((v123 + v123neg).is_zero());
        assert!((v123 + v123 - v246).is_zero());
        assert!((v123 * 2. - v246).is_zero());
    }
}
