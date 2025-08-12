use rayon::prelude::*;
use std::time::Instant;
use core::arch::aarch64::*; // SIMD NEON cho Apple Silicon

const N: usize = 1 << 20; // Kích thước mảng (nằm vừa L2 cache)
const REPEATS: usize = 1_000; // Số vòng lặp để tăng thời gian chạy

fn flops_neon_fma(a: &[f32], b: &[f32]) -> f32 {
    let mut sum_vec = unsafe { vdupq_n_f32(0.0) }; // vector 4 phần tử float

    for _ in 0..REPEATS {
        for chunk in a.chunks_exact(4).zip(b.chunks_exact(4)) {
            let (x, y) = chunk;
            unsafe {
                let vx = vld1q_f32(x.as_ptr());
                let vy = vld1q_f32(y.as_ptr());
                // FMA: sum_vec += vx * vy
                sum_vec = vfmaq_f32(sum_vec, vx, vy);
            }
        }
    }

    // Cộng các phần tử lại thành 1 số float
    let mut sum_arr = [0.0f32; 4];
    unsafe { vst1q_f32(sum_arr.as_mut_ptr(), sum_vec) };
    sum_arr.iter().sum()
}

#[cfg(test)]
mod tests {
    use log::info;
    use utils::log::configuration::init_logger;
    use super::*;
    #[test]
    fn bench_perf() {
        init_logger();
        // Tạo dữ liệu test
        let a: Vec<f32> = vec![1.0; N];
        let b: Vec<f32> = vec![2.0; N];

        // Bắt đầu tính
        let start = Instant::now();
        let result: f32 = (0..num_cpus::get())
            .into_par_iter()
            .map(|_| flops_neon_fma(&a, &b))
            .sum();
        let duration = start.elapsed().as_secs_f64();

        // Số FLOPs:
        //  - mỗi vector có 4 phần tử
        //  - mỗi FMA = 2 FLOPs
        //  - N phần tử × REPEATS × threads × 2 FLOPs
        let threads = num_cpus::get();
        let flops_count = N as f64 * REPEATS as f64 * threads as f64 * 2.0;
        let gflops = flops_count / duration / 1e9;

        info!("Result sum: {}", result);
        info!("Time: {:.3} s", duration);
        info!("GFLOPS: {:.2}", gflops);
    }
}

