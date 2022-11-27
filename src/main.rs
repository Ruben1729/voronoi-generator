mod voronoi;
use voronoi::seed;
use voronoi::export;

const AMOUNT: usize = 20;
const WIDTH:  usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let map: Vec<Vec<u32>> = seed::generate(AMOUNT, WIDTH, HEIGHT);
    export::export_image("test.ppm", map, WIDTH, HEIGHT);
}
