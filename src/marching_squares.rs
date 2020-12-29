use super::quad_tree::*;
use super::util::*;
use num::{Integer, NumCast};

pub struct Path {
    points: Vec<Point<f32>>,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Segment {
    start: Point<f32>,
    end: Point<f32>,
}

enum CellIndex {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

const CELL_OFFSETS: [Point<u32>; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

struct MarchingSquares<'a> {
    img: &'a Image<'a>,
    quad_tree: TreeNode,
}

impl MarchingSquares<'_> {
    pub fn new<'a>(img: &'a Image<'a>) -> MarchingSquares<'a> {
        let quad_tree = TreeNode::create(img);
        MarchingSquares {
            img: img,
            quad_tree: quad_tree,
        }
    }

    fn cell_state(&self, cell: &Point<u32>, threshold: i32) -> u8 {
        // Save the state of the 4 corners of the cell
        // cell_state will be a 4-bit binary number with each digit corresponding to the corner in the order of offsets
        //      with a 0 indicating the corner is below the threshold, a 1 indicating above the threshold
        // cell_state = 0b<top_left><top_right><bottom_left><bottom_right>
        let mut cell_state = 0;
        for offset in &CELL_OFFSETS {
            let corner_state = match self.img.get_val(&(cell + offset)) {
                Some(val) => {
                    if val >= threshold {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            };
            cell_state = (cell_state << 1) + corner_state;
        }
        cell_state
    }

    fn cell_to_segments(&self, cell: &Point<u32>, threshold: i32) -> Vec<Segment> {
        let cell_state = self.cell_state(&cell, threshold);
        let cells: Vec<Point<u32>> = CELL_OFFSETS.iter().map(|offset| cell + offset).collect();
        let vals: Vec<Option<i32>> = cells.iter().map(|coord| self.img.get_val(&coord)).collect();

        let t_bottom = dist_between_option_values(
            threshold as f32,
            &vals[CellIndex::BottomLeft as usize],
            &vals[CellIndex::BottomRight as usize],
        );
        let t_right = dist_between_option_values(
            threshold as f32,
            &vals[CellIndex::TopRight as usize],
            &vals[CellIndex::BottomRight as usize],
        );
        let t_left = dist_between_option_values(
            threshold as f32,
            &vals[CellIndex::BottomLeft as usize],
            &vals[CellIndex::TopLeft as usize],
        );
        let t_top = dist_between_option_values(
            threshold as f32,
            &vals[CellIndex::TopLeft as usize],
            &vals[CellIndex::TopRight as usize],
        );

        let left = || {
            let c = cells[CellIndex::BottomLeft as usize];
            let c2 = cells[CellIndex::TopLeft as usize];

            Point {
                x: c.x as f32,
                y: interpolate(t_left, c.y, c2.y),
            }
        };

        let top = || {
            let c = cells[CellIndex::TopLeft as usize];
            let c2 = cells[CellIndex::TopRight as usize];

            Point {
                x: interpolate(t_top, c.x, c2.x),
                y: c.y as f32,
            }
        };

        let bottom = || {
            let c = cells[CellIndex::BottomLeft as usize];
            let c2 = cells[CellIndex::BottomRight as usize];

            Point {
                x: interpolate(t_bottom, c.x, c2.x),
                y: c.y as f32,
            }
        };

        let right = || {
            let c = cells[CellIndex::TopRight as usize];
            let c2 = cells[CellIndex::BottomRight as usize];

            Point {
                x: c.x as f32,
                y: interpolate(t_right, c.y, c2.y),
            }
        };
        // Segments should go clockwise with above threshold on the inside
        match cell_state {
            // x - x
            // x - o
            0b1110 => vec![Segment {
                start: right(),
                end: bottom(),
            }],
            // o - o
            // o - x
            0b0001 => vec![Segment {
                start: bottom(),
                end: right(),
            }],

            // o - o
            // x - o
            0b0010 => vec![Segment {
                start: left(),
                end: bottom(),
            }],
            // x - x
            // o - x
            0b1101 => vec![Segment {
                start: bottom(),
                end: left(),
            }],

            // o - o
            // x - x
            0b0011 => vec![Segment {
                start: left(),
                end: right(),
            }],
            // x - x
            // o - o
            0b1100 => vec![Segment {
                start: right(),
                end: left(),
            }],

            // o - x
            // o - o
            0b0100 => vec![Segment {
                start: right(),
                end: top(),
            }],
            // x - o
            // x - x
            0b1011 => vec![Segment {
                start: top(),
                end: right(),
            }],

            // o - x
            // o - x
            0b0101 => vec![Segment {
                start: bottom(),
                end: top(),
            }],
            // x - o
            // x - o
            0b1010 => vec![Segment {
                start: top(),
                end: bottom(),
            }],

            // o - x
            // x - x
            0b0111 => vec![Segment {
                start: left(),
                end: top(),
            }],
            // x - o
            // o - o
            0b1000 => vec![Segment {
                start: top(),
                end: left(),
            }],

            // o - x
            // x - o
            0b0110 => {
                let avg_val = vals.iter().flatten().sum::<i32>() as f32 / vals.len() as f32;

                // o - - - x
                // |   x   |
                // x - - - o
                if avg_val > threshold as f32 {
                    vec![
                        Segment {
                            start: left(),
                            end: top(),
                        },
                        Segment {
                            start: right(),
                            end: bottom(),
                        },
                    ]
                } else {
                    // o - - - x
                    // |   o   |
                    // x - - - o
                    vec![
                        Segment {
                            start: right(),
                            end: top(),
                        },
                        Segment {
                            start: left(),
                            end: bottom(),
                        },
                    ]
                }
            }
            // x - o
            // o - x
            0b1001 => {
                let avg_val = vals.iter().flatten().sum::<i32>() as f32 / vals.len() as f32;

                // x - - - o
                // |   x   |
                // o - - - x
                if avg_val > threshold as f32 {
                    vec![
                        Segment {
                            start: top(),
                            end: right(),
                        },
                        Segment {
                            start: bottom(),
                            end: left(),
                        },
                    ]
                } else {
                    // x - - - o
                    // |   o   |
                    // o - - - x
                    vec![
                        Segment {
                            start: bottom(),
                            end: right(),
                        },
                        Segment {
                            start: top(),
                            end: left(),
                        },
                    ]
                }
            }

            // 0b1111 => vec![
            //     Segment {
            //         start: cells[CellIndex::TopLeft as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //         end: cells[CellIndex::TopRight as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //     },
            //     Segment {
            //         start: cells[CellIndex::TopRight as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //         end: cells[CellIndex::BottomRight as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //     },
            //     Segment {
            //         start: cells[CellIndex::BottomRight as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //         end: cells[CellIndex::BottomLeft as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //     },
            //     Segment {
            //         start: cells[CellIndex::BottomLeft as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //         end: cells[CellIndex::TopLeft as usize]
            //             .into_iter()
            //             .map(|x| x as f32)
            //             .collect(),
            //     },
            // ],
            0b1111 | 0b0000 => Vec::new(),
            _ => panic!("Not a valid cell state"),
        }
    }

    fn segments_for_threshold(&self, threshold: i32) -> Vec<Segment> {
        let cells = self.quad_tree.above_threshold(threshold);
        cells
            .iter()
            .map(|cell| self.cell_to_segments(cell, threshold))
            .flatten()
            .collect()
    }
}

fn interpolate<T: Integer + NumCast + Copy>(t: f32, left: T, right: T) -> f32 {
    let left: f32 = num::cast(left).unwrap();
    let right: f32 = num::cast(right).unwrap();
    let length = right - left;
    left + (length * t)
}

fn dist_between_values<T: Integer + NumCast + Copy>(value: f32, start: T, end: T) -> f32 {
    let start: f32 = num::cast(start).unwrap();
    let end: f32 = num::cast(end).unwrap();

    if value == start {
        return 0.0;
    }
    if value == end {
        return 1.0;
    }

    let length = end - start;
    (value - start) / length
}

fn dist_between_option_values<T: Integer + NumCast + Copy>(
    value: f32,
    start: &Option<T>,
    end: &Option<T>,
) -> f32 {
    if start.is_none() {
        return 1.0;
    }

    if end.is_none() {
        return 0.0;
    }

    let start = start.unwrap();
    let end = end.unwrap();

    dist_between_values(value, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path() {
        assert_eq!(1, 1)
    }

    #[test]
    #[rustfmt::skip]
    fn test_cell_state() {
        let data = [
            1, 2, 5, 6, 2, 2, 2, 2, 
            3, 4, 7, 8, 2, 2, 2, 2, 
            3, 3, 3, 3, 4, 4, 4, 4, 
            3, 3, 3, 3, 4, 4, 4, 4,
        ];
        let img = Image::new(&data, 8, 4);
        let marching_squares = MarchingSquares::new(&img);

        assert_eq!(
            marching_squares.cell_state(&Point { x: 0, y: 0 }, 2),
            0b0111
        );
        assert_eq!(
            marching_squares.cell_state(&Point { x: 0, y: 0 }, 1),
            0b1111
        );
        assert_eq!(
            marching_squares.cell_state(&Point { x: 0, y: 0 }, 3),
            0b0011
        );
        assert_eq!(
            marching_squares.cell_state(&Point { x: 0, y: 0 }, 4),
            0b0001
        );

        assert_eq!(
            marching_squares.cell_state(&Point { x: 0, y: 2 }, 3),
            0b1111
        );
    }
    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(0.5, 1, 2), 1.5);
        assert_eq!(interpolate(1.0, 1, 2), 2.0);
        assert_eq!(interpolate(0.0, 1, 2), 1.0);

        assert_eq!(interpolate(0.5, 2, 1), 1.5);
        assert_eq!(interpolate(1.0, 2, 1), 1.0);
        assert_eq!(interpolate(0.0, 2, 1), 2.0);
    }

    #[test]
    fn test_distance_between_values() {
        assert_eq!(dist_between_values(1.5, 1, 2), 0.5);
        assert_eq!(dist_between_values(2.0, 1, 2), 1.0);
        assert_eq!(dist_between_values(1.0, 1, 2), 0.0);

        assert_eq!(dist_between_values(1.5, 2, 1), 0.5);
        assert_eq!(dist_between_values(2.0, 2, 1), 0.0);
        assert_eq!(dist_between_values(1.0, 2, 1), 1.0);

        assert_eq!(dist_between_values(0.8, 2, 0), 0.6);
    }

    #[test]
    #[rustfmt::skip]
    fn test_cell_to_segments() {

        let data = [
            1, 2, 5, 6, 2, 2, 2, 2, 
            3, 4, 7, 8, 2, 2, 2, 2, 
            3, 3, 3, 3, 4, 4, 4, 4, 
            3, 3, 3, 3, 4, 4, 4, 4,
        ];
        let img = Image::new(&data, 8, 4);
        let marching_squares = MarchingSquares::new(&img);

        assert_eq!(marching_squares.cell_to_segments(&Point {x: 0, y: 0}, 2), vec!(Segment {start: Point {x: 0.0, y: 0.5}, end: Point {x: 1.0, y: 0.0}}));
        assert_eq!(marching_squares.cell_to_segments(&Point {x: 0, y: 0}, 3), vec!(Segment {start: Point {x: 0.0, y: 1.0}, end: Point {x: 1.0, y: 0.5}}));
        assert_eq!(marching_squares.cell_to_segments(&Point {x: 0, y: 0}, 4), vec!(Segment {start: Point {x: 1.0, y: 1.0}, end: Point {x: 1.0, y: 1.0}}));
    }

    #[test]
    #[rustfmt::skip]
    fn test_segments_for_threshold() {
        let data = [1, 2, 3, 4, 4, 3, 2, 1,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    3, 4, 5, 6, 6, 5, 4, 3,
                    4, 5, 6, 8, 8, 6, 5, 4,
                    4, 5, 6, 8, 8, 6, 5, 4,
                    3, 4, 5, 6, 6, 5, 4, 3,
                    2, 3, 4, 5, 5, 4, 3, 2,
                    1, 2, 3, 4, 4, 3, 2, 1
        ];
        let img = Image::new(&data, 8, 8);
        let marching_squares = MarchingSquares::new(&img);

        let segments = marching_squares.segments_for_threshold(7);

        for segment in segments {
            println!("{:?}", segment);
        }
    }
}
