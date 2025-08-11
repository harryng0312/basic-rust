use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::env;
use std::time::{Duration, Instant};
use log::info;

/// Create random matrix (row-major) flattened as Vec<f64>
/// shape: rows x cols
fn random_matrix(rows: usize, cols: usize, seed: u64) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut v = vec![0.0f64; rows * cols];
    for x in v.iter_mut() {
        *x = rng.random::<f64>();
    }
    v
}

/// Transpose flattened matrix (rows x cols) -> (cols x rows)
fn transpose(mat: &[f64], rows: usize, cols: usize) -> Vec<f64> {
    let mut t = vec![0.0f64; cols * rows];
    for i in 0..rows {
        for j in 0..cols {
            t[j * rows + i] = mat[i * cols + j];
        }
    }
    t
}

/// Sequential multiplication: A(m x k) * B(k x n) = C(m x n)
fn matmul_seq(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; m * n];
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0f64;
            for t in 0..k {
                sum += a[i * k + t] * b[t * n + j];
            }
            c[i * n + j] = sum;
        }
    }
    c
}

/// Parallel by rows: B is pre-transposed to b_t (n x k) so b_col is a row in b_t
fn matmul_par_rows(a: &[f64], b_t: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; m * n];

    // Each chunk is one row of length n
    c.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
        let a_row = &a[i * k..i * k + k];
        for j in 0..n {
            let b_col = &b_t[j * k..j * k + k]; // column j of B
            let mut sum = 0.0f64;
            for t in 0..k {
                sum += a_row[t] * b_col[t];
            }
            row[j] = sum;
        }
    });

    c
}

/// Parallel by elements: each element computed independently
fn matmul_par_elements(a: &[f64], b_t: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
    let mut c = vec![0.0f64; m * n];

    c.par_iter_mut().enumerate().for_each(|(idx, cell)| {
        let i = idx / n;
        let j = idx % n;
        let a_row = &a[i * k..i * k + k];
        let b_col = &b_t[j * k..j * k + k];
        let mut sum = 0.0f64;
        for t in 0..k {
            sum += a_row[t] * b_col[t];
        }
        *cell = sum;
    });

    c
}

/// Compute FLOPs for m x k times k x n: 2 * m * k * n
fn flops(m: usize, k: usize, n: usize) -> f64 {
    2.0 * (m as f64) * (k as f64) * (n as f64)
}

/// Compare two matrices (flattened) — check max absolute error and relative
fn compare_matrices(refm: &[f64], other: &[f64]) -> (f64, f64) {
    let mut max_abs = 0.0f64;
    let mut max_rel = 0.0f64;
    for (r, o) in refm.iter().zip(other.iter()) {
        let abs = (r - o).abs();
        max_abs = max_abs.max(abs);
        let denom = r.abs().max(1e-12);
        max_rel = max_rel.max(abs / denom);
    }
    (max_abs, max_rel)
}

/// Run provided function `f` `reps` times, return Vec<Duration>
fn bench_run<F>(f: F, reps: usize) -> Vec<Duration>
where
    F: Fn() -> Vec<f64> + Sync,
{
    let mut times = Vec::with_capacity(reps);
    // warmup once
    let _ = f();
    for _ in 0..reps {
        let t0 = Instant::now();
        let _ = f();
        times.push(t0.elapsed());
    }
    times
}

/// Print simple statistics
fn print_stats(times: &[Duration]) {
    let mut secs: Vec<f64> = times.iter().map(|d| d.as_secs_f64()).collect();
    secs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let best = secs.first().cloned().unwrap_or(0.0);
    let median = secs[secs.len() / 2];
    let mean = secs.iter().sum::<f64>() / secs.len() as f64;
    info!("  runs: {}", secs.len());
    info!(
        "  best: {:.6} s, median: {:.6} s, mean: {:.6} s",
        best, median, mean
    );
}

fn parse_arg(args: &[String], key: &str, default: usize) -> usize {
    if let Some(pos) = args.iter().position(|x| x == key) {
        if pos + 1 < args.len() {
            return args[pos + 1].parse().expect("invalid number");
        }
    }
    default
}

#[cfg(test)]
mod tests {
    use utils::log::configuration::init_logger;
    use super::*;

    #[test]
    fn test_perf() {
        // RAYON_NUM_THREADS=8
        init_logger();
        let args: Vec<String> = env::args().collect();
        // usage: cargo run --release -- --m 1024 --k 512 --n 2048 --reps 3 --seed 42
        let m = parse_arg(&args, "--m", 2048);
        let k = parse_arg(&args, "--k", 512);
        let n = parse_arg(&args, "--n", 1024);
        let reps = parse_arg(&args, "--reps", 3);
        let seed = parse_arg(&args, "--seed", 12345) as u64;

        info!("Matrix dims: A {}x{}, B {}x{}, C {}x{}", m, k, k, n, m, n);
        info!("Reps per method: {}", reps);
        if let Ok(threads) = std::env::var("RAYON_NUM_THREADS") {
            info!("RAYON_NUM_THREADS = {}", threads);
        } else {
            info!("RAYON_NUM_THREADS not set (Rayon will pick default threads)");
        }
        info!("Allocating matrices (may use lot of RAM) ...");

        let a = random_matrix(m, k, seed);
        let b = random_matrix(k, n, seed.wrapping_add(1));
        let b_t = transpose(&b, k, n); // shape n x k

        info!("Warmup small block to reduce first-run noise...");
        {
            let small_m = m.min(8);
            let small_k = k.min(8);
            let small_n = n.min(8);
            let _ = matmul_seq(
                &a[..small_m * small_k],
                &b[..small_k * small_n],
                small_m,
                small_k,
                small_n,
            );
        }

        // Sequential
        info!("\n== Sequential ==");
        let seq_closure = || matmul_seq(&a, &b, m, k, n);
        let times_seq = bench_run(seq_closure, reps);
        print_stats(&times_seq);
        let best_seq = times_seq
            .iter()
            .map(|d| d.as_secs_f64())
            .fold(f64::INFINITY, f64::min);
        let gflops_seq = flops(m, k, n) / 1.0e9 / best_seq;
        info!("Best GFLOPS (seq): {:.3}", gflops_seq);

        // Parallel by rows
        info!("\n== Parallel by rows (Rayon) ==");
        let rows_closure = || matmul_par_rows(&a, &b_t, m, k, n);
        let times_rows = bench_run(rows_closure, reps);
        print_stats(&times_rows);
        let best_rows = times_rows
            .iter()
            .map(|d| d.as_secs_f64())
            .fold(f64::INFINITY, f64::min);
        let gflops_rows = flops(m, k, n) / 1.0e9 / best_rows;
        info!("Best GFLOPS (rows): {:.3}", gflops_rows);

        // Parallel by elements
        info!("\n== Parallel by elements (Rayon) ==");
        let elems_closure = || matmul_par_elements(&a, &b_t, m, k, n);
        let times_elems = bench_run(elems_closure, reps);
        print_stats(&times_elems);
        let best_elems = times_elems
            .iter()
            .map(|d| d.as_secs_f64())
            .fold(f64::INFINITY, f64::min);
        let gflops_elems = flops(m, k, n) / 1.0e9 / best_elems;
        info!("Best GFLOPS (elems): {:.3}", gflops_elems);

        // Sanity check: compute one result each (best single-run) to compare values
        info!("\nSanity check (compare some elements) ...");
        let c_seq = matmul_seq(&a, &b, m, k, n);
        let c_rows = matmul_par_rows(&a, &b_t, m, k, n);
        let c_elems = matmul_par_elements(&a, &b_t, m, k, n);

        let (max_abs_r, max_rel_r) = compare_matrices(&c_seq, &c_rows);
        let (max_abs_e, max_rel_e) = compare_matrices(&c_seq, &c_elems);
        info!(
            "rows: max_abs = {:.3e}, max_rel = {:.3e}",
            max_abs_r, max_rel_r
        );
        info!(
            "elems: max_abs = {:.3e}, max_rel = {:.3e}",
            max_abs_e, max_rel_e
        );

        info!("\nDone.");
    }
}
