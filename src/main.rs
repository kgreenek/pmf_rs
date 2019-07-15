fn compute_pmf(p_vector: &Vec<f64>) -> Vec<f64> {
    let n = p_vector.len();
    let mut pmf = vec![0.0; n + 1];
    pmf[0] = 1.0;
    for k in 0..n {
        for kk in (1..(k + 2)).rev() {
            pmf[kk] += p_vector[k] * (pmf[kk - 1] - pmf[kk]);
        }
        pmf[0] *= 1.0 - p_vector[k];
    }
    pmf
}

fn main() {
    const COUNT: usize = 10;
    let n = 15000;
    let p_vector = vec![0.0; n];
    let mut total_ms: u128 = 0;
    for i in 0..COUNT {
        let start_t = std::time::Instant::now();
        compute_pmf(&p_vector);
        let ms = start_t.elapsed().as_millis();
        println!("Loop {} took {} ms", i, ms);
        total_ms += ms;
    }
    println!("Average time: {} ms", (total_ms as f64) / (COUNT as f64));
}
