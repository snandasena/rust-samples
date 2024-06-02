use std::fmt::Debug;
use std::os::unix::raw::time_t;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        1.0f64 / 2.0f64 * self.height * self.length
    }
}

fn print_debug<T: Debug>(t: &T)
{
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}


pub fn test_generic_bounds()
{
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&triangle); // the trait `Debug` is not implemented for `Triangle`
    println!("Area: {}", area(&triangle));
}