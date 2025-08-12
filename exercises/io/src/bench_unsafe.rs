// flops.rs
use std::env;
use std::thread;
use std::time::Instant;

fn parse_usize(arg: Option<&String>, default: usize) -> usize {
    arg.and_then(|s| s.parse::<usize>().ok()).unwrap_or(default)
}

fn run_f64(iters: u64) -> f64 {
    // local variables to maximize in-register usage
    let a: f64 = 1.2345;
    let b: f64 = 6.7890;
    // accumulator stored in stack, flushed to volatile at end
    let mut c: f64 = 0.0;

    for _ in 0..iters {
        // one multiply and one add => 2 FLOPs
        c += a * b;
    }

    // prevent optimizer from removing the loop
    unsafe {
        let p: *mut f64 = &mut c;
        std::ptr::write_volatile(p, std::ptr::read_volatile(p));
    }
    c
}

fn run_f32(iters: u64) -> f32 {
    let a: f32 = 1.2345;
    let b: f32 = 6.7890;
    let mut c: f32 = 0.0;

    for _ in 0..iters {
        c += a * b;
    }

    unsafe {
        let p: *mut f32 = &mut c;
        std::ptr::write_volatile(p, std::ptr::read_volatile(p));
    }
    c
}

#[cfg(test)]
mod test_bench_unsafe {
    use super::*;
    use log::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_bench_perf() {
        init_logger();
        let args: Vec<String> = env::args().collect();

        // usage: flops [f32|f64] [threads] [iters_per_thread]
        let precision = args.get(1).map(|s| s.as_str()).unwrap_or("f64");
        let n_threads = parse_usize(args.get(2), num_cpus::get());
        let iters_per_thread = parse_usize(args.get(3), 50_000_000); // default 50M

        info!(
            "Precision: {}, threads: {}, iters/thread: {}",
            precision, n_threads, iters_per_thread
        );

        let ops_per_iter = 2u64; // 1 mul + 1 add = 2 FLOPs per loop iteration

        let start = Instant::now();

        let mut handles = Vec::with_capacity(n_threads);
        for _ in 0..n_threads {
            let iters = iters_per_thread as u64;
            let prec = precision.to_string();
            let handle = thread::spawn(move || {
                if prec == "f32" {
                    let r = run_f32(iters);
                    // return as f64 to aggregate
                    r as f64
                } else {
                    let r = run_f64(iters);
                    r
                }
            });
            handles.push(handle);
        }

        // collect and prevent optimization by summing results
        let mut sum: f64 = 0.0;
        for h in handles {
            let v = h.join().expect("thread panic");
            sum += v;
        }

        let elapsed = start.elapsed();
        let secs = elapsed.as_secs_f64();

        let total_iters = (iters_per_thread as u64) * (n_threads as u64);
        let total_flops = (total_iters as f64) * (ops_per_iter as f64);

        let flops_per_sec = total_flops / secs;
        info!("Elapsed: {:.4} s", secs);
        info!("Total iterations: {}", total_iters);
        info!("Total FLOPs executed: {:.0}", total_flops);
        info!(
            "Measured FLOPS: {:.3} FLOP/s  ({:.3} GFLOPS)",
            flops_per_sec,
            flops_per_sec / 1e9
        );
        // print sum to avoid optimizer removal
        info!("Check sum (ignore): {:.6}", sum);
    }
}
