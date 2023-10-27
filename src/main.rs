use euler::*;
use rand::Rng;
use raylib::prelude::*;

use std::f32::consts::PI;

fn noise(x: f32, y: f32, v: &[[Vec2; CELLS]; CELLS]) -> f32 {
    let side = v.len() as f32;
    let i = ((x * side).floor() as usize).min(v.len() - 1).max(0);
    let ii = i as f32 / side;
    let ii1 = (i + 1) as f32 / side;
    let j = ((y * side).floor() as usize).min(v.len() - 1).max(0);
    let jj = j as f32 / side;
    let jj1 = (j + 1) as f32 / side;
    let mut grid: [[f32; 2]; 2] = [[0.0; 2]; 2];
    grid[0][0] = vec2!(x - ii, y - jj).dot(v[i][j]);
    grid[0][1] = vec2!(x - ii, y - jj1).dot(v[i][(j + 1) % v.len()]);
    grid[1][0] = vec2!(x - ii1, y - jj).dot(v[(i + 1) % v.len()][j]);
    grid[1][1] = vec2!(x - ii1, y - jj1).dot(v[(i + 1) % v.len()][(j + 1) % v.len()]);
    let x = (x - ii) * side;
    let y = (y - jj) * side;

    interp(grid, x, y)
}

fn interp(grid: [[f32; 2]; 2], x: f32, y: f32) -> f32 {
    let x = fade(x);
    let y = fade(y);
    let a = lerp(grid[0][0], grid[1][0], x);
    let b = lerp(grid[0][1], grid[1][1], x);
    lerp(a, b, y)
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // 6t^5 - 15t^4 + 10t^3
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + (b - a) * x
}

const SIDE: i32 = 1000;
const AGENTS: usize = 500;

const CELLS: usize = 5;
const SCALER: usize = 20;

const CELL_SIDE: f32 = SIDE as f32 / (CELLS as f32 * SCALER as f32);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SIDE, SIDE)
        .title("perlin_flow_field")
        .build();

    let mut v = [[vec2![]; CELLS]; CELLS];

    let mut rng = rand::thread_rng();
    for i in 0..CELLS {
        for j in 0..CELLS {
            let x = rng.gen::<f32>() - 0.5;
            let y = rng.gen::<f32>() - 0.5;
            v[i][j] = vec2!(x, y).normalize();
        }
    }

    let mut agents: Vec<Vec2> = vec![];

    for _ in 0..AGENTS {
        let x = rng.gen::<f32>() * SIDE as f32;
        let y = rng.gen::<f32>() * SIDE as f32;
        agents.push(vec2!(x, y));
    }

    let mut flow = [[vec2!(); CELLS * SCALER]; CELLS * SCALER];
    let mut col = [[Color::new(0, 0, 0, 0); CELLS * SCALER]; CELLS * SCALER];

    let mut noise_min: f32 = f32::MAX;
    let mut noise_max: f32 = f32::MIN;

    for i_usize in 0..(CELLS * SCALER) {
        for j_usize in 0..(CELLS * SCALER) {
            let i = (i_usize as f32 + 0.5) / (CELLS * SCALER) as f32;
            let j = (j_usize as f32 + 0.5) / (CELLS * SCALER) as f32;
            let per = noise(i, j, &v);
            noise_max = noise_max.max(per);
            noise_min = noise_min.min(per);
        }
    }
    for i_usize in 0..(CELLS * SCALER) {
        for j_usize in 0..(CELLS * SCALER) {
            let i = (i_usize as f32 + 0.5) / (CELLS * SCALER) as f32;
            let j = (j_usize as f32 + 0.5) / (CELLS * SCALER) as f32;
            let mut per = (noise(i, j, &v) - noise_min) / (noise_max - noise_min);
            let c: u8 = (per.min(1.0).max(0.0) * 255.0) as u8;
            col[i_usize][j_usize] = Color::new(c, c, c, 255);

            per *= 2.0 * PI;
            flow[i_usize][j_usize] = vec2!(per.sin(), per.cos());
        }
    }

    let draw_color = Color::new(128, 0, 0, 255);

    // let mut l: f32 = 0.0;
    rl.set_target_fps(100);
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        // d.clear_background(Color::RED);

        // for i in 0..(CELLS * SCALER) {
        //     for j in 0..(CELLS * SCALER) {
        //         let cell_x = i as f32 * CELL_SIDE;
        //         let cell_y = j as f32 * CELL_SIDE;

        //         d.draw_rectangle(
        //             cell_x as i32 + 1,
        //             cell_y as i32 + 1,
        //             CELL_SIDE as i32 - 2,
        //             CELL_SIDE as i32 - 2,
        //             col[i][j],
        //         );
        //         // let v_len = 10.0f32;
        //         // d.draw_line(
        //         //     cell_centre_x as i32,
        //         //     cell_centre_y as i32,
        //         //     (cell_centre_x + flow[i][j].x * v_len)
        //         //         .min(SIDE as f32)
        //         //         .max(0.0) as i32,
        //         //     (cell_centre_y + flow[i][j].y * v_len)
        //         //         .min(SIDE as f32)
        //         //         .max(0.0) as i32,
        //         //     Color::WHITE,
        //         // );
        //     }
        // }
        for agent in &mut agents {
            d.draw_pixel(agent.x as i32, agent.y as i32, draw_color);

            let vel = flow[(agent.x / CELL_SIDE).min((CELLS * SCALER) as f32).max(0.0) as usize]
                [(agent.y / CELL_SIDE).min((CELLS * SCALER) as f32).max(0.0) as usize];

            // println!(
            //     "pos: {} {} -> {}",
            //     (agent.x / CELL_SIDE).min(CELLS as f32) as usize,
            //     (agent.y / CELL_SIDE).min(CELLS as f32) as usize,
            //     vel
            // );
            let dt = 1.0f32;

            agent.x += vel.x * dt;
            agent.y += vel.y * dt;

            if agent.x > SIDE as f32 {
                agent.x = rng.gen::<f32>() * SIDE as f32;
                agent.y = rng.gen::<f32>() * SIDE as f32;
            }
            if agent.x < 0.0 {
                agent.x = rng.gen::<f32>() * SIDE as f32;
                agent.y = rng.gen::<f32>() * SIDE as f32;
            }
            if agent.y > SIDE as f32 {
                agent.x = rng.gen::<f32>() * SIDE as f32;
                agent.y = rng.gen::<f32>() * SIDE as f32;
            }
            if agent.y < 0.0 {
                agent.x = rng.gen::<f32>() * SIDE as f32;
                agent.y = rng.gen::<f32>() * SIDE as f32;
            }
        }
    }
}
