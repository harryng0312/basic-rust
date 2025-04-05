use ndarray::{Array1, Array2, ArrayBase, CowArray};
use ndarray_rand::RandomExt;
use std::ops::Mul;

use rand;
use rand::Rng;

use crate::number_utils::calc_euclidean;

fn create_ndarray() {
    const NO_ROWS: usize = 7;
    const NO_COLS: usize = 5;
    let input_size = 784;
    let output_size = 2048;
    let mut rng = rand::thread_rng();
    let limit = (6.0 / (input_size + output_size) as f32).sqrt();
    let a: Array2<i32> = Array2::<i32>::from_shape_fn((NO_ROWS, NO_COLS), |(_, i)| {
        // rng.gen_range(-limit..=limit)
        rng.gen_range(0..100)
    });

    println!("a matrix: {:?}\ntranspose:{:?}", a, a.t());
    // let row1: Array1<f64> = a.row(1).map(|&x| x as f64).to_owned();
    // let row0: Array1<f64> = a.row(0).map(|&x| x as f64).to_owned();
    let row1: Array1<f64> = a.row(1).map(|&x| x as f64);
    let row0: Array1<f64> = a.row(0).map(|&x| x as f64);
    // let row0_reshaped = row0.into_shape_clone((NO_COLS, 1)).unwrap();
    // let row1_reshaped = row1.into_shape_clone((1, NO_COLS)).unwrap();
    let row0_reshaped = row0.to_shape((NO_COLS, 1)).unwrap();
    let row1_reshaped = row1.to_shape((1, NO_COLS)).unwrap();
    println!(
        "Khoảng cách Euclidean: {:?}\ndot product:{:?}\nvector prod:{:?}\nouter prod:{:?}",
        calc_euclidean(&row0, &row1),
        // a.dot(&a.t())
        // Array2::dot(&a, &a.t())
        &a.dot(&a.t()),
        &row0.dot(&row1),
        &row0_reshaped * &row1_reshaped,
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
