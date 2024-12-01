use tch::Tensor;
/// ```sh
/// $ export LIBTORCH=
/// $ export LIBTORCH_USE_PYTORCH=1
/// $ export CARGO_PROFILE_TEST_BUILD_OVERRIDE_DEBUG=true
/// $ export LIBTORCH=$(`path_to_libtorch`)
/// $ export LIBTORCH_BYPASS_VERSION_CHECK=1
/// ```

#[test]
fn test_touch() {
    // Tạo tensor
    let x = Tensor::from_slice(&[1.0, 2.0, 3.0]); // Tensor 1D
    let y = Tensor::from_slice(&[4.0, 5.0, 6.0]);

    // Phép toán tensor
    let z = &x + &y;

    println!("Tensor x: {:?}", x);
    println!("Tensor y: {:?}", y);
    println!("x + y: {:?}", z);
}
