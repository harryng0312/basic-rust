static STASH: i32 = 128;
fn f(p: i32) {
    // STASH = p;
}

#[cfg(test)]
mod tests {
    use crate::basic_lifetime::{f, STASH};

    #[test]
    fn test_lifetime() -> () {
        f(4);
        unsafe {
            println!("{}", STASH);
        }
        ()
    }
}
