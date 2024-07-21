use rand::Rng;

pub fn random_f64(a: f64, b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..b)
}

pub fn random_int(a: i32, b: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..=b)
}
