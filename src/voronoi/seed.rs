use rand::Rng;
//                 0xAAGGBBRR
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
const GRUVBOX_BRIGHT_ORANGE:    u32 = 0xFF1980FE;

const COLORS: [u32; 7] = [GRUVBOX_BRIGHT_RED, GRUVBOX_BRIGHT_GREEN, GRUVBOX_BRIGHT_YELLOW, GRUVBOX_BRIGHT_BLUE, GRUVBOX_BRIGHT_PURPLE, GRUVBOX_BRIGHT_AQUA, GRUVBOX_BRIGHT_ORANGE];

pub fn generate(amount: usize, width: usize, height: usize) -> Vec<Vec<u32>> {
    // Map of pixels and seed locations
    let mut map= vec![vec![0; width]; height];
    let mut seeds = vec![(0,0); amount];

    // Random number generator
    let mut rng = rand::thread_rng();

    // Generate the seeds
    let mut new_seed: (usize, usize);
    for i in 0..amount {
        loop {
            new_seed = (rng.gen_range(0..width), rng.gen_range(0..height));
            if !seeds.contains(&new_seed) {
                seeds[i] = new_seed;
                map[new_seed.0][new_seed.1] = COLORS[rng.gen_range(0..7)];
                break;
            }
        }
    }

    // Generate the diagram
    let mut shortest_distance: usize;
    let mut current_distance: usize;
    for i in 0..width {
        for j in 0..height {
            if map[i][j] != 0 {
                continue;
            }

            shortest_distance = usize::MAX;
            for seed in &seeds {
                current_distance = sqr_distance(seed.0 as isize - i as isize, seed.1 as isize - j as isize);
                if current_distance < shortest_distance {
                    shortest_distance = current_distance;
                    map[i][j] = map[seed.0][seed.1];
                }
            }
        }
    }

    return map;
}

fn sqr_distance(x: isize, y: isize) -> usize {
    return ((x * x) + (y * y)) as usize;
}
