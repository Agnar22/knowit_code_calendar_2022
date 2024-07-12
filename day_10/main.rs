use std::collections::HashMap;

static r_max: f64 = 1e6;
static alpha: f64 = 0.2;
static gamma: f64 = 7.5 * 1e-5;
static lambda: f64 = 83.0;
static beta: f64 = 0.1;

fn r_next(r_inp: i64, u_inp: i64) -> i64 {
    let r_prev: f64 = r_inp as f64;
    let u_prev: f64 = u_inp as f64;

    (r_prev + (alpha*r_prev*(r_max-r_prev))/r_max-gamma*u_prev*r_prev).floor() as i64
}

fn u_next(r_inp: i64, u_inp: i64) -> i64 {
    let r_prev: f64 = r_inp as f64;
    let u_prev: f64 = u_inp as f64;

    (u_prev + (gamma*u_prev*r_prev)/lambda - beta*u_prev).floor() as i64
}


fn main() {
    let visited: HashMap<String, i64> = HashMap::new();

    let mut r: i64 = 125000;
    let mut u: i64 = 3500;
    let i: i64 = 10000000000;
    for j in 0..i {
        let r_n: i64 = r_next(r, u);
        let u_n: i64 = u_next(r, u);
        r = r_n;
        u = u_n;
        visited.get()
    }
    println!("{} {}", r, u);
}
