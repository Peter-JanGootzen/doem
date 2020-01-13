[![Rust-nightly][ri]][rl]

[ri]: https://img.shields.io/badge/rustc-nightly-lightgray.svg
[rl]: https://www.rust-lang.org/

# doem
A simple 3D game built with [luminance](https://github.com/phaazon/luminance-rs), [SPECS](https://github.com/amethyst/specs) and our own math crate: [doem-math](https://github.com/Peter-JanGootzen/doem-math).

Because doem-math was created with const generics, this repo needs Rust nightly to be built(tested uptill nightly 2020-01-08).

[Video](https://youtu.be/_dJZUyysXX0)

Made by Bram-Boris Meerlo and Peter-Jan Gootzen for our final linear algebra assessment.

## Keybindings

W: Move spaceship forward in X direction.

S: Move spaceship backwards in X direction.

D: Move spaceship backwards in Z direction.

A: Move spaceship forward in Z direction.

R: Rotate spaceship forward in local Z direction.

F: Rotate spaceship backwards in local Z direction.

Q: Rotate spaceship backwards in local Y direction.

E: Rotate spaceship forward in local Y direction.

Z: Rotate spaceship forward in local Z direction.

X: Rotate spaceship backwards in local Z direction.

Left: Rotate camera backards in local Y direction.

Right: Rotate camera forward in local Y direction.

PageUp: Increase zoom level.

PageDown: Decrease zoom level.

H: Move camera left in local X direction.

J: Move camera down in local Y direction.

K: Move camera up in local Y direction.

L: Move camera right in local X direction.

LeftShift: Increase thrust.

LeftControl: Decrease thrust.

N: Engine off.

B: Toggle the drawing of bounding boxes and local origins
