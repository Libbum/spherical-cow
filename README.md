<h1 align="center">Spherical Cow</h1>

<div align="center">A high volume fraction sphere packing library</div>
<br />
<div align="center">
    <a href="https://crates.io/crates/spherical-cow">
        <img src="https://img.shields.io/crates/v/spherical-cow.svg" alt="Crates.io" />
    </a>
    │
    <a href="https://docs.rs/spherical-cow/">
        <img src="https://img.shields.io/badge/api-documentation-blue.svg" alt="Docs.rs" />
    </a>
    │
    <a href="https://travis-ci.org/Libbum/spherical-cow">
        <img src="https://travis-ci.org/Libbum/spherical-cow.svg?branch=master" alt="Travis-ci" />
    </a>
    │
    <a href="https://codecov.io/gh/Libbum/spherical-cow">
        <img src="https://codecov.io/gh/Libbum/spherical-cow/branch/master/graph/badge.svg" alt="Codecov" />
    </a>
    │
    <a href="https://app.fossa.io/projects/git%2Bgithub.com%2FLibbum%2Fspherical-cow?ref=badge_shield">
        <img src="https://app.fossa.io/api/projects/git%2Bgithub.com%2FLibbum%2Fspherical-cow.svg?type=shield" alt="FOSSA Status" />
    </a>
</div>
<br />

Based on the advancing fronts algorithm outlined in Valera *et al.*, [Computational Particle Mechanics 2, 161 (2015)](https://doi.org/10.1007/s40571-015-0045-8).

> Milk production at a dairy farm was low, so the farmer wrote to the local university, asking for help from academia.
> A multidisciplinary team of professors was assembled, headed by a theoretical physicist, and two weeks of intensive on-site investigation took place.
> The scholars then returned to the university, notebooks crammed with data, where the task of writing the report was left to the team leader.
> Shortly thereafter the physicist returned to the farm, saying to the farmer, "I have the solution, but it works only in the case of spherical cows in a vacuum".

# Usage

Complete documentation can be found at [docs.rs](https://docs.rs/spherical-cow/).

A simple example to get you packing spheres of radii (0.1..0.2) into a container sphere of radius 2.
```rust
use spherical_cow::shapes::Sphere;
use rand::distributions::Uniform;
use nalgebra::Point3;

fn main() {
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let mut sizes = Uniform::new(0.1, 0.2);

    let spheres = spherical_cow::pack_spheres(boundary, &mut sizes).unwrap();

    println!("Number of spheres: {}", spheres.len());
}
```

More elaborate examples can be found in the [examples](examples/) directory.

# Output

True to its name, it is indeed possible to build a spherical cow:

![spherical cow in vacuum](https://github.com/Libbum/spherical-cow/blob/master/examples/objects/cow_output.jpg?raw=true)

You can run this example yourself from [show_in_cow](examples/show_in_cow.rs).

# Example use cases

The paper which this algorithm comes from gives two examples of real world use cases:

1. Sphere packing a skull model to study fractures due to shocks and penetrating objects.
2. Sphere packing a cutting tool to identify the failure / breaking points when the tool is placed under load.

The reason this library was initially written was to optimise the layout of inflatable [space habitats](https://github.com/Libbum/space-habitats) which may one day be constructed on the Moon and Mars.

# License

Licensed under the Apache License, [Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
These files may not be copied, modified, or distributed except according to those terms.


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FLibbum%2Fspherical-cow.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FLibbum%2Fspherical-cow?ref=badge_large)


## Deeply linked dependencies

The [wayland-protocols](https://github.com/Smithay/wayland-rs/tree/master/wayland-protocols) library (released under an MIT license) is used by [kiss3d](https://github.com/sebcrozet/kiss3d) and [obj](https://github.com/Smithay/wayland-rs/tree/master/wayland-protocols), both of which are only extant in the examples directory of this project (and thus are NOT a part of the library).
Content therein: the file `misc/server-decoration.xml`, is Copyright (C) 2015 Martin Gräßlin and licensed under the GNU Lesser General Public Library, version 2.1. You can find a copy of this license at [https://www.gnu.org/licenses/lgpl-2.1.en.html](https://www.gnu.org/licenses/lgpl-2.1.en.html)

