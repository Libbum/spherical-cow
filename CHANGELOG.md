# Changelog

- 0.1.3
    - Removed nightly dependency (thanks [aleph_0](https://github.com/aleph-oh)).
    - Update to 2018 Edition.
    - Dependencies updated, many were 2 years old.

- 0.1.2

  - Patch release to fix hard failures due to [rust-lang/rust#49799](https://github.com/rust-lang/rust/pull/49799) (thanks [hdhoang](https://github.com/hdhoang))
  - Small performance improvements via algebraic simplifications and minimising vector allocations.
  - Add [itertools](https://github.com/bluss/rust-itertools) to simplify pair collection using `.collect_tuple()`.
  - Added [sphere_pack](benches/sphere_pack.rs) benchmarking target using [criterion](https://github.com/japaric/criterion.rs).
  - Upgrade to [rand v0.5](https://github.com/rust-lang-nursery/rand) which now uses HC-128: increasing our efficiency.

- 0.1.1

  - Complete error handling.
  - Code coverage and testing suite filled out.
  - Add [float-cmp](https://github.com/mikedilger/float-cmp) for more robust floating point comparisons.

- 0.1

  - Initial release.
  - Sphere and cuboid container shapes.
  - Contained trait for user defined shapes.
  - Fully functional in arbitrary geometry using user defined trimeshes.
  - Utility to identify volume of trimeshes using tetrahedron summation.
  - Utility to identify if spheres are contained within trimeshes via ray casting.
  - Volume fraction, void ratio, coordination number and fabric tensors provide quantitative analysis of packing.

