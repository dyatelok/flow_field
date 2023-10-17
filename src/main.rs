use euler::*;
use rand::Rng;
use raylib::prelude::*;

fn perlin(x: f32, y: f32, z: f32, v: &Vec<Vec<Vec<Vec3>>>) -> f32 {
    let x = x * (v.len() - 1) as f32;
    let y = y * (v.len() - 1) as f32;
    let z = z * (v.len() - 1) as f32;

    let i: usize = x.floor() as usize;
    let j: usize = y.floor() as usize;
    let l: usize = z.floor() as usize;

    let ii = i as f32;
    let jj = j as f32;
    let ll = l as f32;

    let xx = x - i as f32;
    let yy = y - j as f32;
    let zz = z - l as f32;

    let mut vv: Vec3;
    let mut g: Vec<Vec<Vec<f32>>> = vec![vec!(vec!(0.0; 2); 2); 2];

    vv = vec3!(x - ii, y - jj, z - ll); //.normalize();
    g[0][0][0] = vv.dot(v[i][j][l]);

    vv = vec3!(x - ii - 1.0, y - jj, z - ll); //.normalize();
    g[1][0][0] = vv.dot(v[i + 1][j][l]);

    vv = vec3!(x - ii, y - jj - 1.0, z - ll); //.normalize();
    g[0][1][0] = vv.dot(v[i][j + 1][l]);

    vv = vec3!(x - ii - 1.0, y - jj - 1.0, z - ll); //.normalize();
    g[1][1][0] = vv.dot(v[i + 1][j + 1][l]);

    vv = vec3!(x - ii, y - jj, z - ll - 1.0); //.normalize();
    g[0][0][1] = vv.dot(v[i][j][l + 1]);

    vv = vec3!(x - ii - 1.0, y - jj, z - ll - 1.0); //.normalize();
    g[1][0][1] = vv.dot(v[i + 1][j][l + 1]);

    vv = vec3!(x - ii, y - jj - 1.0, z - ll - 1.0); //.normalize();
    g[0][1][1] = vv.dot(v[i][j + 1][l + 1]);

    vv = vec3!(x - ii - 1.0, y - jj - 1.0, z - ll - 1.0); //.normalize();
    g[1][1][1] = vv.dot(v[i + 1][j + 1][l + 1]);

    interp(g, xx, yy, zz)
}

fn interp(g: Vec<Vec<Vec<f32>>>, x: f32, y: f32, z: f32) -> f32 {
    let xx = fade(x);
    let yy = fade(y);
    let zz = fade(z);
    let a0 = lerp(g[0][0][0], g[1][0][0], xx);
    let b0 = lerp(g[0][1][0], g[1][1][0], xx);
    let c0 = lerp(a0, b0, yy);
    let a1 = lerp(g[0][0][1], g[1][0][1], xx);
    let b1 = lerp(g[0][1][1], g[1][1][1], xx);
    let c1 = lerp(a1, b1, yy);
    lerp(c0, c1, zz)
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // 6t^5 - 15t^4 + 10t^3
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + (b - a) * x
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 600)
        .title("perlin_flow_field")
        .build();
    let mut v: Vec<Vec<Vec<Vec3>>> = Vec::new();
    let r: usize = 30 + 1;
    let mut rng = rand::thread_rng();
    v.resize(r, Vec::new());
    for i in 0..r {
        v[i].resize(r, Vec::new());
        for j in 0..r {
            v[i][j].resize(r, vec3!());
            for l in 0..r {
                let mut x: f32 = rng.gen();
                let mut y: f32 = rng.gen();
                let mut z: f32 = rng.gen();
                x -= 0.5;
                y -= 0.5;
                z -= 0.5;
                v[i][j][l] = vec3!(x, y, z).normalize();
            }
        }
    }

    let mut V: Vec<Vec2> = vec![];

    for _ in 0..100 {
        let mut x: f32 = rng.gen();
        x *= 600.0;
        let mut y: f32 = rng.gen();
        y *= 600.0;
        V.push(vec2!(x, y));
    }

    let mut per: f32;
    let mut l: f32 = 0.0;
    let mut i: f32;
    let mut j: f32;
    let mut W = [[(vec2!(), vec2!()); 30]; 30];
    let mut w;
    rl.set_target_fps(100);
    let mut k: bool = true;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if k {
            d.clear_background(Color::WHITE);
            k = false;
        }
        //d.clear_background(Color::WHITE);
        for I in 0..30 {
            for J in 0..30 {
                i = I as f32;
                j = J as f32;
                per = perlin(i / 600.0, j / 600.0, l / 600.0, &v) * 2.0 * PI as f32;
                W[I][J].1 = vec2!(per.sin(), per.cos());
                W[I][J].0 += W[I][J].1 / 2.0;
                W[I][J].0 /= W[I][J].0.length();
                //d.draw_line_ex(Vector2{ x : i * 20.0 + 10.0, y : j * 20.0 + 10.0}, Vector2{ x : i * 20.0 + 20.0 * per.sin() + 10.0, y : j * 20.0 + 20.0 * per.cos() + 10.0},3.0,Color::BLACK);
                for v in &V {
                    d.draw_pixel(v.x as i32, v.y as i32, Color::new(128, 128, 128, 255));
                }
            }
        }
        for vv in &mut V {
            w = W[(vv.x / 20.0).min(29.0) as usize][(vv.y / 20.0).min(29.0) as usize].0;
            vv.x += w.x; //* 5.0;
            vv.y += w.y; //* 5.0;
            if vv.x > 600.0 {
                vv.x -= 600.0;
            }
            if vv.x < 0.0 {
                vv.x += 600.0;
            }

            if vv.y > 600.0 {
                vv.y -= 600.0;
            }
            if vv.y < 0.0 {
                vv.y += 600.0;
            }
        }
        l += 0.1;
        if l > 599.0 {
            l = 0.0;
        }
    }
}
