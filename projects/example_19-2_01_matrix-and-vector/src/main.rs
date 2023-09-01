use std::ops::{Add, Mul};

fn main() {
    let p = Point {
        x: 2, y: 3
    };
    let m1 = Matrix {
        attr_11: 5,
        attr_12: 6,
        attr_21: 7,
        attr_22: 8,
    };
    let m2 = Matrix {
        attr_11: 1,
        attr_12: 0,
        attr_21: 0,
        attr_22: 1,
    };
    println!("m1 * m2 = {:?}", m1 * m2);
    println!("m1 * p = {:?}", m1 * p);
    println!("m2 * p = {:?}", m2 * p);
}

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug, Clone, Copy)]
struct Matrix<T> {
    attr_11: T,
    attr_12: T,
    attr_21: T,
    attr_22: T,
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let attr_11 = self.attr_11 * rhs.attr_11 + self.attr_12 * rhs.attr_21;
        let attr_12 = self.attr_11 * rhs.attr_12 + self.attr_12 * rhs.attr_22;
        let attr_21 = self.attr_21 * rhs.attr_11 + self.attr_22 * rhs.attr_21;
        let attr_22 = self.attr_21 * rhs.attr_12 + self.attr_22 * rhs.attr_22;
        Self::Output {
            attr_11,
            attr_12,
            attr_21,
            attr_22,
        }
    }
}

impl<T> Mul<Point<T>> for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Copy,
{
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        let x = self.attr_11 * rhs.x + self.attr_12 * rhs.y;
        let y = self.attr_21 * rhs.x + self.attr_22 * rhs.y;
        Self::Output { x, y }
    }
}
