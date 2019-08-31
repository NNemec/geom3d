# geom3d

A Rust library for geometry in three dimensions

## Design goals

### Maximal performance by designing for SIMD from the start

Optimal SIMD performance depends on the data layout. My main focus is Intel SSE, but the principle is the same for all similar architectures:
SIMD vectors group registers by powers of two. For 3d graphics, a 4-vector is an obvious match to store the three 'x', 'y' and 'z' components,
leaving 1 spare register, commonly called 'w' which can be used in various ways.

The four components within a SIMD vector are indexed 0..3. On Intel SSE, there are two sets of commands: parallel ("P") and serial ("S"). Parallel commands
act on all four components, serial commands only on component 0. Since the leftover 'w' component of our xyzw quartett is the only one with a special meaning,
it is the one where individual access is most valuable. For the default data layout of our vectors we therefore choose (w,x,y,z).

Even though the 'w' component may not always be useful, we will always keep it in place to preserve the same efficient data layout throughout our code.

### Optimal usability for 3d geometry handing in games etc.

Geometry in 3d has very different demands than 2d. Trying to cover both with the same library requires compromise and adds complexity. This library focuses
on 3d geometry and only bring in aspects from other dimensionalities where needed.

Geometry in general is a huge mathematical field which we will attempt to cover. This library is intended for the very specific use case of computer games
and including graphics as well as physics simulation. In this field, single precision floating point arithmetic is sufficient for nearly all use cases.

... to be continued ...