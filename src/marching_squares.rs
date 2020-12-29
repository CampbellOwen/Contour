use super::quad_tree::*;
use super::util::*;
use num::{Integer, NumCast};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Path {
    points: Vec<Point<f32>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Segment {
    start: Point<f32>,
    end: Point<f32>,
    cell_coord: Point<u32>,
    direction: Direction,
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
                cell_coord: *cell,
                direction: Direction::Down,
            }],
            // o - o
            // o - x
            0b0001 => vec![Segment {
                start: bottom(),
                end: right(),
                cell_coord: *cell,
                direction: Direction::Right,
            }],

            // o - o
            // x - o
            0b0010 => vec![Segment {
                start: left(),
                end: bottom(),
                cell_coord: *cell,
                direction: Direction::Down,
            }],
            // x - x
            // o - x
            0b1101 => vec![Segment {
                start: bottom(),
                end: left(),
                cell_coord: *cell,
                direction: Direction::Left,
            }],

            // o - o
            // x - x
            0b0011 => vec![Segment {
                start: left(),
                end: right(),
                cell_coord: *cell,
                direction: Direction::Down,
            }],
            // x - x
            // o - o
            0b1100 => vec![Segment {
                start: right(),
                end: left(),
                cell_coord: *cell,
                direction: Direction::Right,
            }],

            // o - x
            // o - o
            0b0100 => vec![Segment {
                start: right(),
                end: top(),
                cell_coord: *cell,
                direction: Direction::Up,
            }],
            // x - o
            // x - x
            0b1011 => vec![Segment {
                start: top(),
                end: right(),
                cell_coord: *cell,
                direction: Direction::Right,
            }],

            // o - x
            // o - x
            0b0101 => vec![Segment {
                start: bottom(),
                end: top(),
                cell_coord: *cell,
                direction: Direction::Up,
            }],
            // x - o
            // x - o
            0b1010 => vec![Segment {
                start: top(),
                end: bottom(),
                cell_coord: *cell,
                direction: Direction::Down,
            }],

            // o - x
            // x - x
            0b0111 => vec![Segment {
                start: left(),
                end: top(),
                cell_coord: *cell,
                direction: Direction::Up,
            }],
            // x - o
            // o - o
            0b1000 => vec![Segment {
                start: top(),
                end: left(),
                cell_coord: *cell,
                direction: Direction::Left,
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
                            cell_coord: *cell,
                            direction: Direction::Up,
                        },
                        Segment {
                            start: right(),
                            end: bottom(),
                            cell_coord: *cell,
                            direction: Direction::Down,
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
                            cell_coord: *cell,
                            direction: Direction::Up,
                        },
                        Segment {
                            start: left(),
                            end: bottom(),
                            cell_coord: *cell,
                            direction: Direction::Down,
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
                            cell_coord: *cell,
                            direction: Direction::Right,
                        },
                        Segment {
                            start: bottom(),
                            end: left(),
                            cell_coord: *cell,
                            direction: Direction::Left,
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
                            cell_coord: *cell,
                            direction: Direction::Right,
                        },
                        Segment {
                            start: top(),
                            end: left(),
                            cell_coord: *cell,
                            direction: Direction::Left,
                        },
                    ]
                }
            }

            0b1111 | 0b0000 => Vec::new(),
            _ => panic!("Not a valid cell state"),
        }
    }

    fn segments_for_threshold(&self, threshold: i32) -> HashMap<Point<u32>, Vec<Segment>> {
        let mut segment_map: HashMap<Point<u32>, Vec<Segment>> = HashMap::new();
        let cells = self.quad_tree.above_threshold(threshold);
        for cell in cells
            .iter()
            .map(|cell| self.cell_to_segments(cell, threshold))
        {
            if let Some(segment) = cell.get(0) {
                segment_map.insert(segment.cell_coord, cell);
            }
        }

        segment_map
    }

    fn get_next_segment<'a>(
        cell_segments: &'a HashMap<Point<u32>, Vec<Segment>>,
        segment: &Segment,
    ) -> Option<&'a Segment> {
        let cell_diff: Point<i32> = match segment.direction {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        };

        let next_cell_coord: Point<u32> = Point {
            x: (segment.cell_coord.x as i32 + cell_diff.x) as u32,
            y: (segment.cell_coord.y as i32 + cell_diff.y) as u32,
        };

        if let Some(next_segments) = cell_segments.get(&next_cell_coord) {
            for seg in next_segments {
                if segment.end == seg.start {
                    return Some(seg);
                }
            }
        }

        None
    }

    fn trace_segments(cell_segments: &HashMap<Point<u32>, Vec<Segment>>) -> Vec<Path> {
        let mut paths = Vec::new();
        let mut visited_segments: HashMap<Segment, bool> = HashMap::new();

        for (cell_coord, segments) in cell_segments {
            for segment in segments {
                if visited_segments.contains_key(&segment) {
                    continue;
                }
                let mut curr_path: Vec<Point<f32>> = vec![segment.start, segment.end];
                visited_segments.insert(*segment, true);
                let mut curr_seg = MarchingSquares::get_next_segment(cell_segments, segment);
                while let Some(next_segment) = curr_seg {
                    if visited_segments.contains_key(next_segment) {
                        paths.push(Path { points: curr_path });
                        curr_path = Vec::new();
                        curr_seg = None;
                    } else {
                        curr_path.push(next_segment.end);
                        curr_seg = MarchingSquares::get_next_segment(cell_segments, segment);
                    }
                }

                if curr_path.len() > 0 {
                    paths.push(Path { points: curr_path });
                }
            }
        }

        paths
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

        assert_eq!(
            marching_squares.cell_to_segments(&Point { x: 0, y: 0 }, 2),
            vec!(Segment {
                start: Point { x: 0.0, y: 0.5 },
                end: Point { x: 1.0, y: 0.0 },
                cell_coord: Point { x: 0, y: 0 },
                direction: Direction::Up
            })
        );
        assert_eq!(
            marching_squares.cell_to_segments(&Point { x: 0, y: 0 }, 3),
            vec!(Segment {
                start: Point { x: 0.0, y: 1.0 },
                end: Point { x: 1.0, y: 0.5 },
                cell_coord: Point { x: 0, y: 0 },
                direction: Direction::Down
            })
        );
        assert_eq!(
            marching_squares.cell_to_segments(&Point { x: 0, y: 0 }, 4),
            vec!(Segment {
                start: Point { x: 1.0, y: 1.0 },
                end: Point { x: 1.0, y: 1.0 },
                cell_coord: Point { x: 0, y: 0 },
                direction: Direction::Right
            })
        );
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

        for segment in &segments {
            println!("{:?}", segment);
        }
        assert_eq!(segments.len(), 8);
    }

    #[test]
    fn test_next_segment() {
        let data = [
            1, 2, 3, 4, 4, 3, 2, 1, 2, 3, 4, 5, 5, 4, 3, 2, 3, 4, 5, 6, 6, 5, 4, 3, 4, 5, 6, 8, 8,
            6, 5, 4, 4, 5, 6, 8, 8, 6, 5, 4, 3, 4, 5, 6, 6, 5, 4, 3, 2, 3, 4, 5, 5, 4, 3, 2, 1, 2,
            3, 4, 4, 3, 2, 1,
        ];
        let img = Image::new(&data, 8, 8);
        let marching_squares = MarchingSquares::new(&img);

        let segments = marching_squares.segments_for_threshold(7);
        let segment = &segments[&Point { x: 2, y: 2 }][0];

        assert_eq!(
            MarchingSquares::get_next_segment(&segments, segment),
            Some(&segments[&Point { x: 3, y: 2 }][0])
        );
    }
}
