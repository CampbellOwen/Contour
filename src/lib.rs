use wasm_bindgen::prelude::*;
mod marching_squares;
mod quad_tree;
mod util;

use std::fmt;

use serde::{Deserialize, Serialize};

use marching_squares::{MarchingSquares, Path};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvgPath {
    pub class: String,
    pub path: String,
    pub fill: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Svg {
    pub view_box: String,
    pub paths: Vec<SvgPath>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn isoline(data: &[i32], width: u32, height: u32, thresholds: &[i32]) -> JsValue {
    console_error_panic_hook::set_once();
    let svg = isoline_to_svg(data, width, height, thresholds);
    JsValue::from_serde(&svg).unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn isoline(data: &[i32], width: u32, height: u32, thresholds: &[i32]) -> Svg {
    isoline_to_svg(data, width, height, thresholds)
}

fn isoline_to_svg(data: &[i32], width: u32, height: u32, thresholds: &[i32]) -> Svg {
    let image = util::Image::new(data, width, height);
    let marching_squares = MarchingSquares::new(&image);

    let threshold_to_path = |threshold: &i32| {
        let isoline = marching_squares.isoline(*threshold);
        let path: String = isoline
            .paths
            .iter()
            .map(|path| path_to_svg_path(path).join(" "))
            .collect::<Vec<String>>()
            .join(" ");
        SvgPath {
            class: format!("threshold_{}_path", threshold),
            fill: "none".to_string(),
            path: path,
        }
    };

    Svg {
        view_box: format!("0 0 {} {}", width, height),
        paths: thresholds
            .iter()
            .map(threshold_to_path)
            .collect::<Vec<SvgPath>>(),
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![format!(
            "<svg viewBox=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.view_box
        )];

        for path in &self.paths {
            lines.push(format!(
                "\t<path fill=\"none\" stroke=\"black\" stroke-width=\"1\" class=\"{}\" d=\"{}\" />",
                path.class, path.path
            ));
        }
        lines.push("</svg>".to_string());

        write!(f, "{}", lines.join("\n"))
    }
}

pub fn path_to_svg_path(path: &Path) -> Vec<String> {
    let mut svg_path = vec![format!("M{},{}", &path.points[0].x, &path.points[0].y)];

    for point in &path.points[1..] {
        svg_path.push(format!("L{},{}", point.x, point.y));
    }

    if path.circular {
        svg_path.push("Z".to_string());
    }
    svg_path
}

#[cfg(test)]
mod tests {

    use super::marching_squares::IsolineLayer;
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_isoline() {
        let data = [
            1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1, 
            2, 3, 4, 5, 5, 4, 3, 2, 2, 3, 4, 5, 5, 4, 3, 2,  
            3, 4, 5, 6, 6, 5, 4, 3, 3, 4, 5, 6, 6, 5, 4, 3,  
            4, 5, 6, 8, 8, 6, 5, 4, 4, 5, 6, 8, 8, 6, 5, 4, 
            4, 5, 6, 8, 8, 6, 5, 4, 4, 5, 6, 8, 8, 6, 5, 4, 
            3, 4, 5, 6, 6, 5, 4, 3, 3, 4, 5, 6, 6, 5, 4, 3, 
            2, 3, 4, 5, 5, 4, 3, 2, 2, 3, 4, 5, 5, 4, 3, 2, 
            1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1,
            1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1, 
            2, 3, 4, 5, 5, 4, 3, 2, 2, 3, 4, 5, 5, 4, 3, 2, 
            3, 4, 5, 6, 6, 5, 4, 3, 3, 4, 5, 6, 6, 5, 4, 3, 
            4, 5, 6, 8, 8, 6, 5, 4, 4, 5, 6, 8, 8, 6, 5, 4, 
            4, 5, 6, 8, 8, 6, 5, 4, 4, 5, 6, 8, 8, 6, 5, 4, 
            3, 4, 5, 6, 6, 5, 4, 3, 3, 4, 5, 6, 6, 5, 4, 3, 
            2, 3, 4, 5, 5, 4, 3, 2, 2, 3, 4, 5, 5, 4, 3, 2, 
            1, 2, 3, 4, 4, 3, 2, 1, 1, 2, 3, 4, 4, 3, 2, 1,
        ];


        println!("{}", isoline(&data, 16, 16, &vec![7]));
        println!("{}", isoline(&data, 16, 16, &vec![5]));
        println!("{}", isoline(&data, 16, 16, &vec![3]));
        println!("{}", isoline(&data, 16, 16, &vec![3,5,7]));

        let image = util::Image::new(&data, 16, 16);
        let marching_squares = MarchingSquares::new(&image);
        let IsolineLayer{paths, threshold:_} = marching_squares.isoline(5);

        for path in paths {
            println!("{:?}", path);
        }
    }
}
