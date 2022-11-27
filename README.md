# Introduction

The idea of this project was just to get familiar with the language rust.

A voronoi diagram is essentially a way of breaking up a plane. There are seeds randomly placed throughout the plane with colors assigned to each of them. Each pixel then gets assigned the color of the seed closest to them.

Once the plane is generated with the appropriate colors, the pixels are exported to a ppm image in the format P6.

To run the program simply run the following command in the shell

```shell
C:\voronoi-generator cargo run
```

This will generate the image ``test.ppm`` which the user can then open through gimp or some other image editing software.

# Reference
[Wikipedia Reference](https://en.wikipedia.org/wiki/Voronoi_diagram)
[Rust](https://www.rust-lang.org/learn)
