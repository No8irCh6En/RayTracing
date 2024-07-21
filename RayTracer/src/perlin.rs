use crate::rt::{random_f64, random_int};
use crate::vec3::Vec3;
use rand::Rng;
static POINT_COUNT: i32 = 256;
pub struct Perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    fn generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = (0..POINT_COUNT).collect();
        Perlin::permute(&mut p);
        p
    }

    fn permute(p: &mut Vec<i32>) {
        for i in (1..p.len()).rev() {
            let target = random_int(0, i as i32);
            p.swap(i, target as usize);
        }
    }

    pub fn new() -> Self {
        let randvec: Vec<_> = (0..POINT_COUNT)
            .map(|_| {
                Vec3::new(
                    random_f64(0.0, 1.0),
                    random_f64(0.0, 1.0),
                    random_f64(0.0, 1.0),
                )
            })
            .collect();
        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        self.perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(&self, c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize]
                        * Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }
        accum.abs()
    }
}
