use std::iter::FromIterator;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add + Copy> Add<Point<T>> for Point<T> {
    type Output = Point<T::Output>;
    fn add(self, other: Point<T>) -> Point<T::Output> {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl<T: Add + Copy> Add<&Point<T>> for Point<T> {
    type Output = Point<T::Output>;
    fn add(self, other: &Point<T>) -> Point<T::Output> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Add + Copy> Add<Point<T>> for &Point<T> {
    type Output = Point<T::Output>;

    fn add(self, other: Point<T>) -> Point<T::Output> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Add + Copy> Add<&Point<T>> for &Point<T> {
    type Output = Point<T::Output>;

    fn add(self, other: &Point<T>) -> Point<T::Output> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a Point<T> {
    type Item = T;
    type IntoIter = PointIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator {
            point: self,
            index: 0,
        }
    }
}

pub struct PointIterator<'a, T> {
    point: &'a Point<T>,
    index: usize,
}

impl<'a, T: Copy> Iterator for PointIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let result = match self.index {
            0 => self.point.x,
            1 => self.point.y,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<T> FromIterator<T> for Point<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut it = iter.into_iter();
        Point {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
        }
    }
}

pub struct Image<'a> {
    data: &'a [i32],
    pub width: u32,
    pub height: u32,
}

impl Image<'_> {
    pub fn new(data: &[i32], width: u32, height: u32) -> Image {
        Image {
            data,
            width,
            height,
        }
    }

    pub fn get_val(&self, pt: &Point<u32>) -> Option<i32> {
        if pt.x >= self.width || pt.y >= self.height {
            return None;
        }

        Some(self.data[point_to_index(pt, self.width)])
    }
}

fn point_to_index(point: &Point<u32>, width: u32) -> usize {
    assert!(point.x < width); // 0 indexed
    ((width * point.y) + point.x) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point_to_index() {
        let pt = Point { x: 0, y: 2 };
        assert_eq!(point_to_index(&pt, 1), 2);
        assert_eq!(point_to_index(&pt, 3), 6);
    }
}
