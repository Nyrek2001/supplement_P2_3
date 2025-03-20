use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Implementing the Add trait for Point3D
impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_addition() {
        let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
        let p2 = Point3D { x: 4.0, y: 5.0, z: 6.0 };
        let result = p1 + p2;
        assert_eq!(result, Point3D { x: 5.0, y: 7.0, z: 9.0 });
    }
}
