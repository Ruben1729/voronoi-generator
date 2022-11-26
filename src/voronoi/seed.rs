use rand::Rng;

const WHITE: u32 = 0xFFFFFFFF;
const RED:   u32 = 0xFF0000FF;
const GREEN: u32 = 0xFF00FF00;
const BLUE:  u32 = 0xFFFF0000;

const GRUVBOX_BRIGHT_RED:       u32 = 0xFF3449FB;
const GRUVBOX_BRIGHT_GREEN:     u32 = 0xFF26BBB8;
const GRUVBOX_BRIGHT_YELLOW:    u32 = 0xFF2FBDFA;
const GRUVBOX_BRIGHT_BLUE:      u32 = 0xFF98A583;
const GRUVBOX_BRIGHT_PURPLE:    u32 = 0xFF9B86D3;
const GRUVBOX_BRIGHT_AQUA:      u32 = 0xFF7CC08E;
const GRUVBOX_BRIGHT_ORANGE:    u32 = 0xFF1980FE;// AAGGBBRR (r, g, b, a)

const COLORS: [u32; 7] = [GRUVBOX_BRIGHT_RED, GRUVBOX_BRIGHT_GREEN, GRUVBOX_BRIGHT_YELLOW, GRUVBOX_BRIGHT_BLUE, GRUVBOX_BRIGHT_PURPLE, GRUVBOX_BRIGHT_AQUA, GRUVBOX_BRIGHT_ORANGE];
const AMOUNT: usize = 10;

pub fn generate(width: usize, height: usize) -> Vec<u32> {
    let mut _map: Vec<u32> = Vec::with_capacity(width * height);
    let mut seeds: [(usize, usize); AMOUNT] = [(0,0); 10];
    let mut rng = rand::thread_rng();
    let mut rn;
    let mut rc = 0;

    for _i in 0..(width * height) {
        _map.push(0);
    }

    let mut amount = AMOUNT;
    while amount > 0 {
        rn = rng.gen_range(0..((width * height) - 1));
        if _map[rn] != 1 {
            _map[rn] = COLORS[rc%7];
            rc = rc + 1;
            seeds[amount - 1] = get_coordinates(rn, width);
            amount = amount - 1;
        }
    }

    let mut shortestDist: usize = 0;
    let mut dist: usize = 0;
    let mut closestSeed: usize = 0;
    let mut currentCoords: (usize, usize) = (0, 0);
    for i in 0..(width * height) {
        currentCoords = get_coordinates(i, width);
        shortestDist = sqr_distance(seeds[0].0 - currentCoords.0, seeds[0].1 - currentCoords.1);
        closestSeed = 0;
        for (j ,seed) in seeds.iter().enumerate() {
            currentCoords = get_coordinates(i, width);
            dist = sqr_distance(seed.0 - currentCoords.0, seed.1 - currentCoords.1);
            if dist < shortestDist {
                shortestDist = dist;
                closestSeed = j;
            }
        }

        _map[i] = _map[get_index(seeds[closestSeed], width)];
    }

    return _map;
}

fn sqr_distance(x: usize, y: usize) -> usize {
    return (x * x) + (y * y);
}

fn get_coordinates(i: usize, width: usize) -> (usize, usize) {
    println!("{}", i%width);
    return (i % width, i / width);
}

fn get_index(seed: (usize, usize), width: usize) -> usize {
    return seed.0 + width * seed.1;
}
