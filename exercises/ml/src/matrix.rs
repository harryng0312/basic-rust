use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::time::Instant;

/// Create random matrix (row-major) flattened as Vec<f64>
fn random_matrix(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut v = vec![0.0f64; n * n];
    for x in v.iter_mut() {
        // *x = rng.gen::<f64>();
        *x = rng.random::<f64>();
    }
    v
}

/// Transpose flattened n x n matrix (row-major) -> returns flattened transpose
fn transpose(mat: &[f64], n: usize) -> Vec<f64> {
    let mut t = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..n {
            t[j * n + i] = mat[i * n + j];
        }
    }
    t
}

/// Sequential (naive) matrix multiplication: C = A * B, all flattened row-major
fn matmul_seq(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; n * n];
    // for cache-friendly access, B is not transposed here
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0f64;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
    c
}

/// Parallel by rows: each row computed in parallel. Pre-transpose B for locality.
fn matmul_par_rows(a: &[f64], b_t: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; n * n];
    // iterate rows of A (i), compute row i of C
    c.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
        let a_row = &a[i * n..i * n + n];
        for j in 0..n {
            // b_t[j * n .. j * n + n] is B column j as a slice (row in b_t)
            let b_col = &b_t[j * n..j * n + n];
            let mut sum = 0.0f64;
            // inner product
            for k in 0..n {
                sum += a_row[k] * b_col[k];
            }
            row[j] = sum;
        }
    });
    c
}

/// Parallel by element: operate on flattened C elements in parallel.
/// Uses B transposed for locality as well.
fn matmul_par_elements(a: &[f64], b_t: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; n * n];

    c.par_iter_mut().enumerate().for_each(|(idx, cell)| {
        let i = idx / n;
        let j = idx % n;
        let a_row = &a[i * n..i * n + n];
        let b_col = &b_t[j * n..j * n + n];
        let mut sum = 0.0f64;
        for k in 0..n {
            sum += a_row[k] * b_col[k];
        }
        *cell = sum;
    });

    c
}

/// Compute FLOPs for NxN matrix multiply: 2 * n^3 (mul + add)
fn flops(n: usize) -> f64 {
    2.0 * (n as f64).powi(3)
}

#[cfg(test)]
mod tests {
    use log::info;
    use utils::log::configuration::init_logger;
    use super::*;
    #[test]
    fn perform_matrix_test() {
        init_logger();
        let args: Vec<String> = env::args().collect();
        // usage: cargo run --release -- --size 1024
        let mut n: usize = 2048; //1024;
        if args.len() >= 3 && args[1] == "--size" {
            n = args[2].parse().expect("invalid size");
        }
        info!("Matrix size: {} x {}", n, n);

        // allocate A, B
        info!("Allocating matrices...");
        let a = random_matrix(n);
        let b = random_matrix(n);
        let b_t = transpose(&b, n);

        // Warm up to mitigate first-run effects
        info!("Warmup sequential (small)...");
        {
            let _ = matmul_seq(&a[..(n.min(64) * n)], &b[..(n.min(64) * n)], n.min(64));
        }

        // 1) sequential
        info!("\n=== Sequential ===");
        let t0 = Instant::now();
        let c_seq = matmul_seq(&a, &b, n);
        let dt = t0.elapsed();
        let secs = dt.as_secs_f64();
        info!("Time: {:.3} s", secs);
        info!("GFLOPS: {:.3}", flops(n) / 1.0e9 / secs);

        // 2) parallel by rows
        info!("\n=== Parallel by rows (rayon) ===");
        let t0 = Instant::now();
        let c_rows = matmul_par_rows(&a, &b_t, n);
        let dt = t0.elapsed();
        let secs = dt.as_secs_f64();
        info!("Time: {:.3} s", secs);
        info!("GFLOPS: {:.3}", flops(n) / 1.0e9 / secs);

        // 3) parallel by elements
        info!("\n=== Parallel by elements (rayon) ===");
        let t0 = Instant::now();
        let c_elems = matmul_par_elements(&a, &b_t, n);
        let dt = t0.elapsed();
        let secs = dt.as_secs_f64();
        info!("Time: {:.3} s", secs);
        info!("GFLOPS: {:.3}", flops(n) / 1.0e9 / secs);

        // Sanity check: compare a few entries
        info!("\nSanity check: c_seq[0..5] vs c_rows[0..5] vs c_elems[0..5]");
        for k in 0..5 {
            info!("{} | {} | {}", c_seq[k], c_rows[k], c_elems[k]);
        }
    }
}
