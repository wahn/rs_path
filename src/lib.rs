/*!
# path

To construct a `Path` you should use `PathBuilder`.

## Using **path**

A path is basically used to distribute arbitrary many points along a
polyline (number of points connected along a path through those points
via straight lines).

```.rust
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
```
*/

extern crate nalgebra as na;

use na::Vec4;

#[derive(Debug)]
pub struct Path {
    points: Vec<Vec4<f32>>,
    lvalues: Vec<f32>,
    changed: bool,
    current_length: f32,
}

impl Path {
    /// Distribute `numpts` many points along a polyline.
    pub fn evaluate(&mut self, numpts: u8) -> Vec<Vec4<f32>>
    {
        let mut points: Vec<Vec4<f32>> = Vec::new();
        let length = self.length();
        let step = length / ((numpts-1) as f32);
        let mut current_length: f32 = 0.0;
        let mut index = 0;
        for _i in 0..numpts {
            while index + 1 != self.lvalues.len() &&
                current_length > self.lvalues[index+1] {
                index += 1;
            }
            if (index + 1) == self.lvalues.len() {
                index -= 1;
            }
            // divide found distance
            let p1 = self.points[index];
            let p2 = self.points[index+1];
            let a = current_length - self.lvalues[index];
            let b = self.lvalues[index+1] - current_length;
            let point = (p2 * a + p1 * b) /
                (self.lvalues[index+1] - self.lvalues[index]);
            points.push(point);
            current_length += step;
        }
        points
    }
    /// Calculate the length of a polyline by summing up the length of
    /// all individual vectors.
    pub fn length(&mut self) -> f32
    {
        let mut length = 0.0;
        if self.changed {
            self.lvalues.push(0.0);
            for i in 1..self.points.len() {
                let vector: Vec4<f32> = self.points[i] - self.points[i-1];
                length += na::norm(&vector);
                self.lvalues.push(length);
            }
            self.current_length = length;
            self.changed = false;
        } else {
            length = self.current_length;
        }
        length
    }
}

/// Helper to construct a Path.

pub struct PathBuilder {
    points: Vec<Vec4<f32>>,
    sorted: Vec<Vec4<f32>>,
    params: Vec<f32>,
}

impl PathBuilder {
    /// Prepares the creation of a path. Points can be added either in
    /// order or by providing an additional parameter (which creates
    /// that order once all points were added).
    pub fn new() -> PathBuilder
    {
        PathBuilder { points: Vec::new(),
                      sorted: Vec::new(),
                      params: Vec::new(),}
    }
    /// Add points in a particular order by repeatedly calling this
    /// function.
    pub fn add_point(&mut self, point: Vec4<f32>) ->
        &mut PathBuilder
    {
        self.points.push(point);
        self
    }
    /// Add points by calling this function repeatedly in any order,
    /// but implicitly defining an order by providing a parameter for
    /// each point.
    pub fn add_sorted_point(&mut self, point: Vec4<f32>, param: f32) ->
        &mut PathBuilder
    {
        if self.sorted.is_empty() {
            self.sorted.push(point);
            self.params.push(param);
        } else {
            // find index to insert param
            let mut was_inserted = false;
            for i in 0..self.params.len() {
                if param < self.params[i] {
                    self.sorted.insert(i, point);
                    self.params.insert(i, param);
                    was_inserted = true;
                    break;
                }
            }
            if !was_inserted {
                // append to end
                self.sorted.push(point);
                self.params.push(param);
            }
        }
        self
    }
    /// Use either points which were added in that particular order or
    /// use provided parameters to sort points added in arbitrary
    /// order.
    pub fn finalize(self) -> Path
    {
        if self.params.is_empty() {
            Path { points: self.points,
                   lvalues: Vec::new(),
                   changed: true,
                   current_length: 0.0f32, }
        } else {
            Path { points: self.sorted,
                   lvalues: Vec::new(),
                   changed: true,
                   current_length: 0.0f32, }
        }
    }
}
