use ndarray::Array2;
use ndarray_rand::RandomExt;

use rand;
use rand::Rng;

use crate::number_utils::calc_euclidean;

fn create_ndarray() {
    const NO_ROWS: usize = 10;
    const NO_COLS: usize = 7;
    let input_size = 784;
    let output_size = 2048;
    let mut rng = rand::thread_rng();
    let limit = (6.0 / (input_size + output_size) as f32).sqrt();
    let a: Array2<i32> = Array2::<i32>::from_shape_fn((NO_COLS, NO_ROWS), |(_, i)| {
        // rng.gen_range(-limit..=limit)
        rng.gen_range(0..100)
    });
    println!(
        "Euclidean: {}",
        calc_euclidean(
            a.row(0).map(|x| { *x as f64 }).to_owned(),
            a.row(1).map(|x| { *x as f64 }).to_owned()
        )
    );
}

#[cfg(test)]
mod tests {
    use crate::nd_array::create_ndarray;

    #[test]
    fn test_ndarray() {
        create_ndarray();
    }
}
