use std::cmp;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    x: u32,
    y: u32
}

use std::ops::Add;
impl Add<Point> for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Add<&Point> for Point {
    type Output = Point;
    
    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub struct TreeNode {
    origin: Point,
    lower_bound: i32,
    upper_bound: i32,
    width: u32,
    height: u32,

    top_left: Option<Box<TreeNode>>,
    top_right: Option<Box<TreeNode>>,
    bottom_left: Option<Box<TreeNode>>,
    bottom_right: Option<Box<TreeNode>>,
}

pub struct Image<'a> {
    data: &'a[i32],
    width: u32,
    height: u32
}

impl Image<'_> {
    fn get_val(&self, pt: &Point) -> Option<i32> {
        if pt.x >= self.width || pt.y >= self.height {
            return None;
        }

        Some(self.data[point_to_index(pt, self.width)])
    }
}

fn point_to_index(point: &Point, width: u32) -> usize {
    assert!(point.x < width); // 0 indexed
    ((width * point.y) + point.x) as usize
}

impl TreeNode {
    fn create(img: &Image, origin: Point, width: u32, height: u32) -> TreeNode {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        if width <= 2 && height <= 2 {
            let mut values: Vec<i32> = Vec::new();
            let offsets: [(u32, u32); 4] = [(0,0), (0, 1), (1, 0), (1,1)];

            for (x,y) in &offsets {
                let point = &origin + Point {x: *x, y: *y};

                img.get_val(&point).map(|val| values.push(val));
            }

            return TreeNode {
                origin: origin,
                lower_bound: *values.iter().min().unwrap_or(&i32::MAX),
                upper_bound: *values.iter().max().unwrap_or(&i32::MIN),
                width: width,
                height: height,
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None
            }
        }
        
        let mut top_right: Option<Box<TreeNode>> = None;
        let mut bottom_left: Option<Box<TreeNode>> = None;
        let mut bottom_right: Option<Box<TreeNode>> = None;

        let next_width = if width <= 2 { width } else { (width + 1) / 2 };
        let next_height = if height <= 2 { height } else { (height + 1) / 2 };


        let mid_y = origin.y + next_height;
        let bottom_height = height - next_height;

        let top_left_tree = TreeNode::create(img, origin.clone(), next_width, next_height);
        min = cmp::min(min, top_left_tree.lower_bound);
        max = cmp::max(max, top_left_tree.upper_bound);
        let top_left = Some(Box::new(top_left_tree));

        if width > 2 {
            let mid_x = origin.x + next_width;
            let right_width = width - next_width;
            let top_right_tree = TreeNode::create(img, Point {x: mid_x, y: origin.y}, right_width, next_height);
            min = if min < top_right_tree.lower_bound {min} else {top_right_tree.lower_bound};
            max = if max > top_right_tree.upper_bound {max} else {top_right_tree.upper_bound};

            top_right = Some(Box::new(top_right_tree));

            if height > 2 {
                let bottom_right_tree = TreeNode::create(img, Point {x: mid_x, y: mid_y}, right_width, bottom_height);
                min = if min < bottom_right_tree.lower_bound {min} else {bottom_right_tree.lower_bound};
                max = if max > bottom_right_tree.upper_bound {max} else {bottom_right_tree.upper_bound};

                bottom_right = Some(Box::new(bottom_right_tree));
            }
        }
        
        if height > 2 {
            let bottom_left_tree = TreeNode::create(img, Point {x: origin.x, y: mid_y}, next_width, bottom_height);
            min = if min < bottom_left_tree.lower_bound {min} else {bottom_left_tree.lower_bound};
            max = if max > bottom_left_tree.upper_bound {max} else {bottom_left_tree.upper_bound};

            bottom_left = Some(Box::new(bottom_left_tree));
        }

        TreeNode {
            origin: origin,
            lower_bound: min,
            upper_bound: max,
            width: width,
            height: height,
            top_left,
            top_right,
            bottom_left,
            bottom_right
        }
    }

    fn under_threshold(&self, threshold: i32) -> Vec<Point> {
        if self.lower_bound > threshold {
            return Vec::new();
        }

        let mut cells: Vec<Point> = Vec::new();

        if self.width <= 2 && self.height <= 2 && self.lower_bound <= threshold {
            cells.push(self.origin.clone());
            return cells;
        }

        let threshold_func = |node: &Box<TreeNode>| node.under_threshold(threshold);
        cells.extend(self.top_left.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(self.top_right.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(self.bottom_left.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(self.bottom_right.as_ref().map_or(Vec::new(), &threshold_func));

        return cells;
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_point_to_index() {
        let pt = Point { x: 0, y: 2};
        assert_eq!(point_to_index(&pt, 1), 2);
        assert_eq!(point_to_index(&pt, 3), 6);
    }

    #[test]
    fn test_create_uneven_quadtree() {
        let data: [i32; 9] = [1,2, 3,
                              4,5, 6,

                              7,8, 9];
        let img = Image {data: &data, width: 3, height: 3};
        let tree = TreeNode::create(&img, Point {x: 0, y: 0}, 3, 3);

        assert_eq!(tree.lower_bound, 1);
        assert_eq!(tree.upper_bound, 9);

        let top_left = tree.top_left.unwrap();
        let top_right = tree.top_right.unwrap();
        let bottom_left = tree.bottom_left.unwrap();
        let bottom_right = tree.bottom_right.unwrap();

        assert_eq!(top_left.width, 2);
        assert_eq!(top_right.width, 1);
        assert_eq!(bottom_left.width, 2);
        assert_eq!(bottom_right.width, 1);

        assert_eq!(top_left.height, 2);
        assert_eq!(top_right.height, 2);
        assert_eq!(bottom_left.height, 1);
        assert_eq!(bottom_right.height, 1);

        assert_eq!(top_left.upper_bound, 5);
        assert_eq!(top_right.upper_bound, 6);
        assert_eq!(bottom_left.upper_bound, 8);
        assert_eq!(bottom_right.upper_bound, 9);

        assert_eq!(top_left.lower_bound, 1);
        assert_eq!(top_right.lower_bound, 3);
        assert_eq!(bottom_left.lower_bound, 7);
        assert_eq!(bottom_right.lower_bound, 9);
    }

    #[test]
    fn test_create_even_quadtree() {
        let data = [1,1,1,1, 2,2,2,2, 
                    1,1,1,1, 2,2,2,2, 

                    3,3,3,3, 4,4,4,4, 
                    3,3,3,3, 4,4,4,4];
        let img = Image {data: &data, width: 8, height: 4};
        let tree = TreeNode::create(&img, Point {x: 0, y: 0}, 8, 4);

        assert_eq!(tree.lower_bound, 1);
        assert_eq!(tree.upper_bound, 4);

        let top_left = tree.top_left.unwrap();
        let top_right = tree.top_right.unwrap();
        let bottom_left = tree.bottom_left.unwrap();
        let bottom_right = tree.bottom_right.unwrap();

        assert_eq!(top_left.width, 4);
        assert_eq!(top_right.width, 4);
        assert_eq!(bottom_left.width, 4);
        assert_eq!(bottom_right.width, 4);

        assert_eq!(top_left.height, 2);
        assert_eq!(top_right.height, 2);
        assert_eq!(bottom_left.height, 2);
        assert_eq!(bottom_right.height, 2);

        assert_eq!(top_left.upper_bound, 1);
        assert_eq!(top_right.upper_bound, 2);
        assert_eq!(bottom_left.upper_bound, 3);
        assert_eq!(bottom_right.upper_bound, 4);

        assert_eq!(top_left.lower_bound, 1);
        assert_eq!(top_right.lower_bound, 2);
        assert_eq!(bottom_left.lower_bound, 3);
        assert_eq!(bottom_right.lower_bound, 4);
    }

    #[test]
    fn test_under_threshold()
    {
        let data = [1,2,5,6, 2,2,2,2, 
                    3,4,7,8, 2,2,2,2, 

                    3,3,3,3, 4,4,4,4, 
                    3,3,3,3, 4,4,4,4];
        let img = Image {data: &data, width: 8, height: 4};
        let tree = TreeNode::create(&img, Point {x: 0, y: 0}, 8, 4);

        let mut cells = tree.under_threshold(2);
        assert_eq!(cells.len(), 3);

        cells = tree.under_threshold(3);

        assert_eq!(cells.len(), 5);
        assert_eq!(cells[0], Point {x: 0, y: 0});
        assert_eq!(cells[1], Point {x: 4, y: 0});
        assert_eq!(cells[2], Point {x: 6, y: 0});
        assert_eq!(cells[3], Point {x: 0, y: 2});
        assert_eq!(cells[4], Point {x: 2, y: 2});
    }
}