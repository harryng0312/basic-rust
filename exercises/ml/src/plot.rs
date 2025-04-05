use ndarray::{Array2, ArrayBase};
use ndarray_rand::RandomExt;
use std::ops::Mul;

use rand;
use rand::Rng;

use crate::number_utils::calc_euclidean;

fn create_scratter() {}

#[cfg(test)]
mod tests {
    use crate::plot::create_scratter;

    #[test]
    fn test_scratter_plot() {
        create_scratter();
    }
}
