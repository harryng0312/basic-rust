use log::{info, log};
use std::ops::{Add, Deref, Mul};
use std::process::Output;

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}
#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,
    /// Imaginary portion of the complex number
    im: T,
}

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[test]
fn test_ov() {
    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
    println!("Passed")
}

struct Person {
    id: u64,
    name: String,
}

struct Student(Person);

struct Student2 {
    person: Person,
    class_id: u32,
}

impl Deref for Student {
    type Target = Person;
    fn deref(&self) -> &Self::Target {
        info!("Deref 1");
        &self.0
    }
}

impl Deref for Student2 {
    type Target = Person;
    fn deref(&self) -> &Self::Target {
        info!("Deref 2");
        &self.person
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_struct::{Person, Student};
    use log::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_multi_deref() {
        init_logger();
        let s = Student(Person {
            id: 1,
            name: "tests".to_string(),
        });
        info!("{}", s.name);
    }
}
