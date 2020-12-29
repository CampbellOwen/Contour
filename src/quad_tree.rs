use super::util::*;
use std::cmp;

pub struct TreeNode {
    origin: Point<u32>,
    lower_bound: i32,
    upper_bound: i32,
    width: u32,
    height: u32,

    top_left: Option<Box<TreeNode>>,
    top_right: Option<Box<TreeNode>>,
    bottom_left: Option<Box<TreeNode>>,
    bottom_right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn create(img: &Image) -> TreeNode {
        create_node(img, Point { x: 0, y: 0 }, img.width, img.height)
    }

    pub fn under_threshold(&self, threshold: i32) -> Vec<Point<u32>> {
        if self.lower_bound > threshold {
            return Vec::new();
        }

        let mut cells: Vec<Point<u32>> = Vec::new();

        if self.width <= 2 && self.height <= 2 && self.lower_bound <= threshold {
            cells.push(self.origin.clone());
            return cells;
        }

        let threshold_func = |node: &Box<TreeNode>| node.under_threshold(threshold);
        cells.extend(self.top_left.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(self.top_right.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(
            self.bottom_left
                .as_ref()
                .map_or(Vec::new(), &threshold_func),
        );
        cells.extend(
            self.bottom_right
                .as_ref()
                .map_or(Vec::new(), &threshold_func),
        );

        cells
    }

    pub fn above_threshold(&self, threshold: i32) -> Vec<Point<u32>> {
        if self.upper_bound < threshold {
            return Vec::new();
        }

        let mut cells: Vec<Point<u32>> = Vec::new();

        if self.width <= 2 && self.height <= 2 && self.upper_bound >= threshold {
            cells.push(self.origin.clone());
            return cells;
        }

        let threshold_func = |node: &Box<TreeNode>| node.above_threshold(threshold);
        cells.extend(self.top_left.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(self.top_right.as_ref().map_or(Vec::new(), &threshold_func));
        cells.extend(
            self.bottom_left
                .as_ref()
                .map_or(Vec::new(), &threshold_func),
        );
        cells.extend(
            self.bottom_right
                .as_ref()
                .map_or(Vec::new(), &threshold_func),
        );

        cells
    }
}

fn create_node(img: &Image, origin: Point<u32>, width: u32, height: u32) -> TreeNode {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    if width <= 2 && height <= 2 {
        let mut values: Vec<i32> = Vec::new();
        let offsets: [(u32, u32); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

        for (x, y) in &offsets {
            let point = &origin + Point { x: *x, y: *y };

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
            bottom_right: None,
        };
    }
    let mut top_right: Option<Box<TreeNode>> = None;
    let mut bottom_left: Option<Box<TreeNode>> = None;
    let mut bottom_right: Option<Box<TreeNode>> = None;

    let next_width = if width <= 2 { width } else { (width + 1) / 2 };
    let next_height = if height <= 2 {
        height
    } else {
        (height + 1) / 2
    };

    let mid_y = origin.y + next_height - 1;
    let bottom_height = height + 1 - next_height;

    let top_left_tree = create_node(img, origin.clone(), next_width, next_height);
    min = cmp::min(min, top_left_tree.lower_bound);
    max = cmp::max(max, top_left_tree.upper_bound);
    let top_left = Some(Box::new(top_left_tree));

    if width > 2 {
        let mid_x = origin.x + next_width - 1;
        let right_width = width + 1 - next_width;
        let top_right_tree = create_node(
            img,
            Point {
                x: mid_x,
                y: origin.y,
            },
            right_width,
            next_height,
        );
        min = if min < top_right_tree.lower_bound {
            min
        } else {
            top_right_tree.lower_bound
        };
        max = if max > top_right_tree.upper_bound {
            max
        } else {
            top_right_tree.upper_bound
        };

        top_right = Some(Box::new(top_right_tree));

        if height > 2 {
            let bottom_right_tree = create_node(
                img,
                Point { x: mid_x, y: mid_y },
                right_width,
                bottom_height,
            );
            min = if min < bottom_right_tree.lower_bound {
                min
            } else {
                bottom_right_tree.lower_bound
            };
            max = if max > bottom_right_tree.upper_bound {
                max
            } else {
                bottom_right_tree.upper_bound
            };

            bottom_right = Some(Box::new(bottom_right_tree));
        }
    }
    if height > 2 {
        let bottom_left_tree = create_node(
            img,
            Point {
                x: origin.x,
                y: mid_y,
            },
            next_width,
            bottom_height,
        );
        min = if min < bottom_left_tree.lower_bound {
            min
        } else {
            bottom_left_tree.lower_bound
        };
        max = if max > bottom_left_tree.upper_bound {
            max
        } else {
            bottom_left_tree.upper_bound
        };

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
        bottom_right,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    #[rustfmt::skip]
    fn test_create_uneven_quadtree() {
        let data: [i32; 9] = [1,2,3,
                              4,5,6,
                              7,8,9];
        let img = Image::new(&data, 3, 3);
        let tree = create_node(&img, Point {x: 0, y: 0}, 3, 3);

        assert_eq!(tree.lower_bound, 1);
        assert_eq!(tree.upper_bound, 9);

        let top_left = tree.top_left.unwrap();
        let top_right = tree.top_right.unwrap();
        let bottom_left = tree.bottom_left.unwrap();
        let bottom_right = tree.bottom_right.unwrap();

        assert_eq!(top_left.width, 2);
        assert_eq!(top_right.width, 2);
        assert_eq!(bottom_left.width, 2);
        assert_eq!(bottom_right.width, 2);

        assert_eq!(top_left.height, 2);
        assert_eq!(top_right.height, 2);
        assert_eq!(bottom_left.height, 2);
        assert_eq!(bottom_right.height, 2);

        assert_eq!(top_left.upper_bound, 5);
        assert_eq!(top_right.upper_bound, 6);
        assert_eq!(bottom_left.upper_bound, 8);
        assert_eq!(bottom_right.upper_bound, 9);

        assert_eq!(top_left.lower_bound, 1);
        assert_eq!(top_right.lower_bound, 2);
        assert_eq!(bottom_left.lower_bound, 4);
        assert_eq!(bottom_right.lower_bound, 5);
    }

    #[test]
    #[rustfmt::skip]
    fn test_create_even_quadtree() {
        let data = [1,1,1,1,2,2,2,2, 
                    1,1,1,1,2,2,2,2, 
                    3,3,3,3,4,4,4,4, 
                    3,3,3,3,4,4,4,4];
        let img = Image::new(&data, 8, 4);
        let tree = create_node(&img, Point {x: 0, y: 0}, 8, 4);

        assert_eq!(tree.lower_bound, 1);
        assert_eq!(tree.upper_bound, 4);

        let top_left = tree.top_left.unwrap();
        let top_right = tree.top_right.unwrap();
        let bottom_left = tree.bottom_left.unwrap();
        let bottom_right = tree.bottom_right.unwrap();

        assert_eq!(top_left.width, 4);
        assert_eq!(top_right.width, 5);
        assert_eq!(bottom_left.width, 4);
        assert_eq!(bottom_right.width, 5);

        assert_eq!(top_left.height, 2);
        assert_eq!(top_right.height, 2);
        assert_eq!(bottom_left.height, 3);
        assert_eq!(bottom_right.height, 3);

        assert_eq!(top_left.upper_bound, 1);
        assert_eq!(top_right.upper_bound, 2);
        assert_eq!(bottom_left.upper_bound, 3);
        assert_eq!(bottom_right.upper_bound, 4);

        assert_eq!(top_left.lower_bound, 1);
        assert_eq!(top_right.lower_bound, 1);
        assert_eq!(bottom_left.lower_bound, 1);
        assert_eq!(bottom_right.lower_bound, 1);
    }

    #[test]
    #[rustfmt::skip]
    fn test_under_threshold()
    {
        let data = [1,2,5,6,2,2,2,2, 
                    3,4,7,8,2,2,2,2, 
                    3,3,3,3,4,4,4,4, 
                    3,3,3,3,4,4,4,4];
        let img = Image::new(&data, 8, 4);
        let tree = create_node(&img, Point {x: 0, y: 0}, 8, 4);

        let cells = tree.under_threshold(2);
        assert_eq!(cells.len(), 10);

        let cells = tree.under_threshold(3);

        assert_eq!(cells.len(), 17);


        let data = [1, 2, 3, 4, 4, 3, 2, 1,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    3, 4, 5, 6, 6, 5, 4, 3,
                    4, 5, 6, 7, 7, 6, 5, 4,
                    4, 5, 6, 7, 7, 6, 5, 4,
                    3, 4, 5, 6, 6, 5, 4, 3,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    1, 2, 3, 4, 4, 3, 2, 1
        ];
        let img = Image::new(&data, 8, 8);
        let tree = create_node(&img, Point {x: 0, y: 0}, img.width, img.height);
        let cells = tree.under_threshold(7);
        println!("{:?}", cells);
    }

    #[test]
    #[rustfmt::skip]
    fn test_above_threshold()
    {
        let data = [1, 2, 3, 4, 3, 2, 1,
                    2, 3, 4, 5, 4, 3, 2,
                    3, 4, 5, 6, 5, 4, 3,
                    4, 5, 6, 7, 6, 5, 4,
                    3, 4, 5, 6, 5, 4, 3,
                    2, 3, 4, 5, 4, 3, 2,
                    1, 2, 3, 4, 3, 2, 1
        ];
        let img = Image::new(&data, 7, 7);
        let tree = create_node(&img, Point {x: 0, y: 0}, img.width, img.height);
        let cells = tree.above_threshold(7);

        assert_eq!(cells, vec![
            Point {x: 2, y: 2}, 
            Point {x: 3, y: 2}, 
            Point {x: 2, y: 3}, 
            Point {x: 3, y: 3}]);

        let data = [1, 2, 3, 4, 4, 3, 2, 1,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    3, 4, 7, 7, 6, 5, 4, 3,
                    4, 5, 7, 7, 6, 6, 5, 4,
                    4, 5, 6, 6, 6, 6, 5, 4,
                    3, 4, 5, 6, 6, 5, 4, 3,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    1, 2, 3, 4, 4, 3, 2, 1
        ];
        let img = Image::new(&data, 8, 8);
        let tree = create_node(&img, Point {x: 0, y: 0}, img.width, img.height);
        let cells = tree.above_threshold(7);

        println!("{:?}", cells);
    }
}
