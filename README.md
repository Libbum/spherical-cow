# Spherical Cow

A high volume fraction sphere packing library based on the advancing fronts algorithm outlined in Valera *et al.*, [Computational Particle Mechanics 2, 161 (2015)](https://doi.org/10.1007/s40571-015-0045-8).

> Milk production at a dairy farm was low, so the farmer wrote to the local university, asking for help from academia.
> A multidisciplinary team of professors was assembled, headed by a theoretical physicist, and two weeks of intensive on-site investigation took place.
> The scholars then returned to the university, notebooks crammed with data, where the task of writing the report was left to the team leader.
> Shortly thereafter the physicist returned to the farm, saying to the farmer, "I have the solution, but it works only in the case of spherical cows in a vacuum".

# Usage

Complete documentation can be found at ...

A simple example to get you packing spheres of radii `(0.1..0.2)` into a container sphere of radius 2.
```rust
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use rand::distributions::Range;
use nalgebra::Point3;

fn main() {
    let boundary = Sphere::new(Point3::origin(), 2.0);
    let mut sizes = Range::new(0.1, 0.2);

    let spheres = spherical_cow::pack_spheres(boundary, &mut sizes);

    println!("Number of spheres: {}", spheres.len());
}
```

More elaborate examples can be found in the [examples](examples/) directory.

# Output

True to its name, it is indeed possible to build a spherical cow:

![spherical cow in vacuum](examples/objects/cow_output.png)

You can run this example yourself from [show_in_cow](examples/show_in_cow.rs).

# Example use cases

The paper which this algorithm comes from gives two examples of real world use cases:

1. Sphere packing a skull model to study fractures due to shocks and penetrating objects.
2. Sphere packing a cutting tool to identify the failure / breaking points when the tool is placed under load.

The reason this library was initially written was to optimise the layout of inflatable [space habitats](https://github.com/Libbum/space-habitats) which may one day be constructed on the Moon and Mars.

# License

Licensed under the Apache License, [Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
These files may not be copied, modified, or distributed except according to those terms.
