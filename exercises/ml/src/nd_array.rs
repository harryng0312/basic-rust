use crate::number_utils::calc_euclidean;
use core::f64;
// use linfa::prelude::{Fit, Transformer};
// use linfa_preprocessing::PreprocessingError;
use ndarray::{Array1, Array2};
use rand;
use rand::Rng;
use smartcore::api::{Transformer, UnsupervisedEstimator};
use smartcore::linalg::basic::arrays::Array;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::preprocessing::numerical::{StandardScaler, StandardScalerParameters};
use std::error::Error;

fn min_max_scale(inp_arr: &Array2<f64>) -> Result<Array2<f64>, Box<dyn Error>> {
    // let dataset = Dataset::from(inp_arr);
    // let targets: Array1<usize> = Array1::zeros(inp_arr.shape()[0]);
    // let mut records = inp_arr.clone();
    // let targets = Array::zeros(records.nrows());
    // let dataset = Dataset::new(records, targets);
    // let min_max_params = LinearScaler::min_max_range(0.0, 1.0).fit(&dataset)?;
    //
    // Ok(min_max_params
    //     .transform(dataset)
    //     .map(|x| x as f64)
    //     .collect())

    // for mut col in result.columns() {
    //     let min = col.fold(f64::INFINITY, |a, &b| a.min(b));
    //     let max = col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    //     if (max - min).abs() > std::f64::EPSILON {
    //         // col.mapv_inplace(|x| (x - min) / (max - min));
    //         col.mapv_inplace(|x| (x - min) / (max - min));
    //     }
    // }

    // let mins = inp_arr.fold_axis(Axis(0), f64::MAX, |&a, &b| a.min(b));
    // let maxs = inp_arr.fold_axis(Axis(0), f64::MIN, |&a, &b| a.max(b));
    // let scaled_arr = inp_arr.mapv(|x| {
    //     let col_min = mins[0] as f64;
    //     let col_max = maxs[0] as f64;
    //
    //     (x as f64 - col_min) / (col_max - col_min)
    // });
    // Ok(scaled_arr)
    let data =
        DenseMatrix::from_2d_vec(&inp_arr.rows().into_iter().map(|row| row.to_vec()).collect())?;
    let scaler = StandardScaler::fit(&data, StandardScalerParameters::default())?;
    let transformed_data = scaler.transform(&data)?;

    let (rows, cols) = transformed_data.shape();
    let flat_vec = transformed_data
        .iter()
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    let array2: Array2<f64> =
        Array2::from_shape_vec((rows, cols), flat_vec).expect("Không thể tạo Array2");
    Ok(array2)
}

pub(crate) fn create_ndarray() -> Result<(), Box<dyn Error>> {
    const NO_ROWS: usize = 7;
    const NO_COLS: usize = 5;
    let input_size = 784;
    let output_size = 2048;
    let mut rng = rand::rng();
    let _limit = (6.0 / (input_size + output_size) as f32).sqrt();
    let inp_arr: Array2<i32> = Array2::<i32>::from_shape_fn((NO_ROWS, NO_COLS), |(_, _)| {
        // rng.gen_range(-limit..=limit)
        rng.random_range(0..100)
    });
    println!("a matrix: {:?}\ntranspose:{:?}", inp_arr, inp_arr.t());
    let inp_arr = min_max_scale(&inp_arr.mapv(|x| x as f64))?;
    println!("scaled matrix: {:?}\ntranspose:{:?}", inp_arr, inp_arr.t());
    // let row1: Array1<f64> = a.row(1).map(|&x| x as f64).to_owned();
    // let row0: Array1<f64> = a.row(0).map(|&x| x as f64).to_owned();
    let row1: Array1<f64> = inp_arr.row(1).map(|&x| x as f64);
    let row0: Array1<f64> = inp_arr.row(0).map(|&x| x as f64);
    // let row0_reshaped = row0.into_shape_clone((NO_COLS, 1)).unwrap();
    // let row1_reshaped = row1.into_shape_clone((1, NO_COLS)).unwrap();
    let row0_reshaped = row0.to_shape((NO_COLS, 1)).unwrap();
    let row1_reshaped = row1.to_shape((1, NO_COLS)).unwrap();
    println!(
        "Khoảng cách Euclidean: {:?}\ndot product:{:?}\nvector prod:{:?}\nouter prod:{:?}",
        calc_euclidean(&row0, &row1),
        // a.dot(&a.t())
        // Array2::dot(&a, &a.t())
        &inp_arr.dot(&inp_arr.t()),
        &row0.dot(&row1),
        &row0_reshaped * &row1_reshaped,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::nd_array::create_ndarray;

    #[test]
    fn test_ndarray() {
        _ = create_ndarray();
    }
}
