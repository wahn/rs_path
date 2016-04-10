extern crate path;
extern crate nalgebra as na;

use na::Vec4;

fn main() {
    // create path (with 10 points)
    let mut path = path::PathBuilder::new();
    for i in 0..10 {
        let point = Vec4::new(i as f32, i as f32, 0.0, 1.0); // i i
        path.add_point(point);
    }
    let mut path = path.finalize();
    // calculate length
    println!("path.length() = {}", path.length());
    // evaluate (with 5 points)
    println!("{:?}", path);
    let pts: u8 = 5;
    let points = path.evaluate(pts);
    println!("{:?}", points);
    // evaluate (with 15 points)
    let pts: u8 = 15;
    let points = path.evaluate(pts);
    println!("{:?}", points);
}
