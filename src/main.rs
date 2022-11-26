mod voronoi;
use voronoi::seed;
use voronoi::export;

const WIDTH:  usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let map= seed::generate(WIDTH, HEIGHT);
    export::export_image("test", map, WIDTH, HEIGHT);
}
