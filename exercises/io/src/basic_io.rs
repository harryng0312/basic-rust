#[warn(dead_code)]
fn copy_file() {
    // open src
    // create dest
}

pub fn test_file() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x: u8 = 1;
        let y: u8 = 1;
        // let z: i32 = x - y;
        println!("x:{}", x - y);
    }
}
