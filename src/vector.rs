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

use num_traits::Float;


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


macro_rules! impl_fixed_array_conversions {
    ($V:ident) => {
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
    }
}

macro_rules! impl_vector3 {
    ($V:ident) => {
        impl<F: Float> $V<F> {
            #[inline]
            pub fn from_xyz(x: F, y: F, z: F) -> $V<F> {
                $V { x: x, y: y, z: z, _w: F::zero() }
            }
        }

        impl_fixed_array_conversions!($V);
    };
}

macro_rules! impl_vector3w {
    ($V:ident) => {
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

        impl_fixed_array_conversions!($V);
    };
}




impl_vector3!(Vector0XYZ);
impl_vector3!(VectorXYZ0);
impl_vector3w!(VectorWXYZ);
impl_vector3w!(VectorXYZW);


#[test]
fn test_constructor() {
    type F = f32;
    let x: F = 1.;
    let y: F = 2.;
    let z: F = 3.;
    let w: F = 42.;
    let zero: F = 0.;

    let v1 = Vector0XYZ::from_xyz(x,y,z);
    assert_eq!(v1.x, x);
    assert_eq!(*v1.as_ref(), [zero,x,y,z]);

    let v2 = VectorXYZ0::from_xyz(x,y,z);
    assert_eq!(v2.x, x);
    assert_eq!(*v2.as_ref(), [x,y,z,zero]);

    let v3 = VectorWXYZ::from_xyz(x,y,z);
    assert_eq!(v3.x, x);
    assert_eq!(v3.w, zero);
    assert_eq!(*v3.as_ref(), [zero,x,y,z]);

    let v4 = VectorXYZW::from_xyz(x,y,z);
    assert_eq!(v4.x, x);
    assert_eq!(v4.w, zero);
    assert_eq!(*v4.as_ref(), [x,y,z,zero]);

    let v5 = VectorWXYZ::from_xyzw(x,y,z,w);
    assert_eq!(v5.x, x);
    assert_eq!(v5.w, w);
    assert_eq!(*v5.as_ref(), [w,x,y,z]);

    let v6 = VectorXYZW::from_xyzw(x,y,z,w);
    assert_eq!(v6.x, x);
    assert_eq!(v6.w, w);
    assert_eq!(*v6.as_ref(), [x,y,z,w]);
}
