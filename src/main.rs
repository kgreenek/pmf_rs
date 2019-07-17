extern crate approx;
use core::arch::x86_64::{_MM_SET_FLUSH_ZERO_MODE, _MM_FLUSH_ZERO_ON};

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
    unsafe {
        _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_ON);
    }
    const COUNT: usize = 3;
    const N: usize = 15000;
    for i in 0..11 {
        let p = (i as f64) * 0.1;
        let p_vector = vec![p; N];
        let mut total_ms: u128 = 0;
        for _ in 0..COUNT {
            let start_t = std::time::Instant::now();
            let pmf = compute_pmf(&p_vector);
            let ms = start_t.elapsed().as_millis();
            //println!("Loop {} took {} ms", i, ms);
            let sum: f64 = pmf.iter().sum();
            assert!(approx::relative_eq!(sum, 1.0, epsilon = 0.00001));
            total_ms += ms;
        }
        println!("p {:0.1} took {:0.1} ms", p, (total_ms as f64) / (COUNT as f64));
    }
}
