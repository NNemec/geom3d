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

use num_traits::Zero;


#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorXYZ0<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    _w: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector0XYZ<S> {
    _w: S,
    pub x: S,
    pub y: S,
    pub z: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorXYZW<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct VectorWXYZ<S> {
    pub w: S,
    pub x: S,
    pub y: S,
    pub z: S,
}


macro_rules! impl_fixed_array_conversions {
    ($V:ident) => {
        impl<S> AsRef<[S; 4]> for $V<S> {
            #[inline]
            fn as_ref(&self) -> &[S; 4] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<S> AsMut<[S; 4]> for $V<S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [S; 4] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<S: Copy> Into<[S; 4]> for $V<S> {
            #[inline]
            fn into(self) -> [S; 4] {
                *self.as_ref()
            }
        }

        // can't convert from reference to reference, since array may not be aligned

        impl<S: Copy> From<[S; 4]> for $V<S> {
            #[inline]
            fn from(v: [S; 4]) -> $V<S> {
                unsafe {
                    let mut res: $V<S> = std::mem::uninitialized();
                    *res.as_mut() = v;
                    res
                }
            }
        }
    }
}

macro_rules! impl_vector3 {
    ($V:ident) => {
        impl<S: Zero> $V<S> {
            #[inline]
            pub fn from_xyz(x: S, y: S, z: S) -> $V<S> {
                $V { x: x, y: y, z: z, _w: S::zero() }
            }
        }

        impl_fixed_array_conversions!($V);
    };
}

macro_rules! impl_vector3w {
    ($V:ident) => {
        impl<S: Zero> $V<S> {
            #[inline]
            pub fn from_xyz(x: S, y: S, z: S) -> $V<S> {
                $V { x: x, y: y, z: z, w: S::zero() }
            }

            #[inline]
            pub fn from_xyzw(x: S, y: S, z: S, w: S) -> $V<S> {
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
    type S = f32;
    let x: S = 1.;
    let y: S = 2.;
    let z: S = 3.;
    let w: S = 42.;
    let zero: S = 0.;

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
