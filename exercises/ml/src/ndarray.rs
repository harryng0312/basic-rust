use ndarray::{array, Array2};

#[test]
fn test_ndarray() {
    // Khởi tạo hai ma trận
    let a: Array2<f64> = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];
    let b: Array2<f64> = array![
        [7.0, 8.0],
        [9.0, 10.0],
        [11.0, 12.0]
    ];

    // Nhân hai ma trận
    let result = a.dot(&b);

    // In kết quả
    println!("Matrix A:\n{:?}", a);
    println!("Matrix B:\n{:?}", b);
    println!("A dot B:\n{:?}", result);
    println!("Transpose:\n{:?}", result.t());
}