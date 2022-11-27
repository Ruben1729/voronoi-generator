# Introduction

A voronoi diagram is essentially a way of breaking up a plane. There are seeds randomly placed throughout the plane with colors assigned to each of them. Each pixel then gets assigned the color of the seed closest to them.

Once the plane is generated with the appropriate colors, the pixels are exported to a ppm image in the format P6.

To run the program simply run the following command in the shell

```shell
C:\voronoi-generator cargo run
```

# Reference
[Wikipedia Reference](https://en.wikipedia.org/wiki/Voronoi_diagram)
