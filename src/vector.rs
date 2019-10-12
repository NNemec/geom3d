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

use num_traits::Zero;


#[cfg_attr(feature = "repr_simd", repr(simd))]
pub struct VectorXYZ0<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    _w: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
pub struct Vector0XYZ<S> {
    _w: S,
    pub x: S,
    pub y: S,
    pub z: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
pub struct VectorXYZW<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

#[cfg_attr(feature = "repr_simd", repr(simd))]
pub struct VectorWXYZ<S> {
    pub w: S,
    pub x: S,
    pub y: S,
    pub z: S,
}

macro_rules! impl_vector3 {
    ($VectorN:ident) => {
        impl<S: Zero> $VectorN<S> {
            #[inline]
            pub fn from_xyz(x: S, y: S, z: S) -> $VectorN<S> {
                $VectorN { x: x, y: y, z: z, _w: S::zero() }
            }
        }
    };
}

macro_rules! impl_vector3w {
    ($VectorN:ident) => {
        impl<S: Zero> $VectorN<S> {
            #[inline]
            pub fn from_xyz(x: S, y: S, z: S) -> $VectorN<S> {
                $VectorN { x: x, y: y, z: z, w: S::zero() }
            }

            #[inline]
            pub fn from_xyzw(x: S, y: S, z: S, w: S) -> $VectorN<S> {
                $VectorN { x: x, y: y, z: z, w: w }
            }
        }
    };
}

impl_vector3!(Vector0XYZ);
impl_vector3!(VectorXYZ0);
impl_vector3w!(VectorWXYZ);
impl_vector3w!(VectorXYZW);


#[test]
fn test_constructor() {
    let v1 = Vector0XYZ::from_xyz(1,2,3);
    assert_eq!(v1.x, 1);
    assert_eq!(v1.y, 2);

    let v2 = VectorXYZ0::from_xyz(1,2,3);
    assert_eq!(v2.x, 1);
    assert_eq!(v2.y, 2);

    let v3 = VectorWXYZ::from_xyz(1,2,3);
    assert_eq!(v3.x, 1);
    assert_eq!(v3.w, 0);

    let v4 = VectorXYZW::from_xyz(1,2,3);
    assert_eq!(v4.x, 1);
    assert_eq!(v4.w, 0);

    let v5 = VectorWXYZ::from_xyzw(1,2,3,4);
    assert_eq!(v5.x, 1);
    assert_eq!(v5.w, 4);

    let v6 = VectorXYZW::from_xyzw(1,2,3,4);
    assert_eq!(v6.x, 1);
    assert_eq!(v6.w, 4);
}
