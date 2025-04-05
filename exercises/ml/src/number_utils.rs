use ndarray::{Array1, Array2};

pub fn calc_euclidean(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
    (a - b).mapv(|x| x.powi(2)).sum().sqrt()
}
