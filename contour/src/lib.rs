mod marching_squares;
mod quad_tree;
mod util;

use std::fmt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use marching_squares::{MarchingSquares, Path};
use std::io::Cursor;
use tiff::decoder::*;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

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
pub fn isoline_from_tiff(data: &[u8], thresholds: &[f32]) -> JsValue {
    console_error_panic_hook::set_once();
    let image = bytes_to_image(data).unwrap();
    let svg = isoline_to_svg(&image, thresholds).unwrap();
    JsValue::from_serde(&svg).unwrap()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn isoline(data: Vec<f32>, width: u32, height: u32, thresholds: &[f32]) -> JsValue {
    console_error_panic_hook::set_once();

    let image = util::Image::new(data, width, height);
    let svg = isoline_to_svg(&image, thresholds).unwrap();
    JsValue::from_serde(&svg).unwrap()
}

fn bytes_to_image(data: &[u8]) -> Result<util::Image<f32>, tiff::TiffError> {
    let mut reader = Decoder::new(Cursor::new(data))?;
    let read_result = &reader.read_image()?;
    let (width, height) = &reader.dimensions()?;
    let image_data: Vec<f32> = match read_result {
        DecodingResult::U8(d) => d.iter().map(|x| *x as f32).collect(),
        DecodingResult::U16(d) => d.iter().map(|x| *x as f32).collect(),
        DecodingResult::U32(d) => d.iter().map(|x| *x as f32).collect(),
        DecodingResult::U64(d) => d.iter().map(|x| *x as f32).collect(),
        DecodingResult::F32(d) => d.clone(),
        DecodingResult::F64(d) => d.iter().map(|x| *x as f32).collect(),
    };

    Ok(util::Image::new(image_data, *width, *height))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn isoline_from_tiff(data: &[u8], thresholds: &[f32]) -> Svg {
    let img = bytes_to_image(data).unwrap();
    isoline_to_svg(&img, thresholds).unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn isoline(data: Vec<f32>, width: u32, height: u32, thresholds: &[f32]) -> Svg {
    let image = util::Image::new(data, width, height);
    isoline_to_svg(&image, thresholds).unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
fn isoline_to_svg(img: &util::Image<f32>, thresholds: &[f32]) -> Result<Svg, tiff::TiffError> {
    let marching_squares = MarchingSquares::new(img);

    let threshold_to_path = |(i, threshold): (usize, &f32)| {
        let isoline = marching_squares.isoline(*threshold);
        let path: String = isoline
            .paths
            .iter()
            .map(|path| path_to_svg_path(path).join(" "))
            .collect::<Vec<String>>()
            .join(" ");
        SvgPath {
            class: format!("threshold_{}_path", i),
            fill: "none".to_string(),
            path: path,
        }
    };

    Ok(Svg {
        view_box: format!("0 0 {} {}", img.width, img.height),
        paths: thresholds
            .par_iter()
            .enumerate()
            .map(threshold_to_path)
            .collect::<Vec<SvgPath>>(),
    })
}

#[cfg(target_arch = "wasm32")]
fn isoline_to_svg(img: &util::Image<f32>, thresholds: &[f32]) -> Result<Svg, tiff::TiffError> {
    let marching_squares = MarchingSquares::new(img);

    let threshold_to_path = |(i, threshold): (usize, &f32)| {
        let isoline = marching_squares.isoline(*threshold);
        let path: String = isoline
            .paths
            .iter()
            .map(|path| path_to_svg_path(path).join(" "))
            .collect::<Vec<String>>()
            .join(" ");
        SvgPath {
            class: format!("threshold_{}_path", i),
            fill: "none".to_string(),
            path: path,
        }
    };

    Ok(Svg {
        view_box: format!("0 0 {} {}", img.width, img.height),
        paths: thresholds
            .iter()
            .enumerate()
            .map(threshold_to_path)
            .collect::<Vec<SvgPath>>(),
    })
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
        ].iter().map(|num| *num as f32).collect::<Vec<f32>>();

        let image = util::Image::new(data, 16, 16);


        println!("{}", isoline_to_svg(&image, &vec![7.0]).unwrap());
        println!("{}", isoline_to_svg(&image, &vec![5.0]).unwrap());
        println!("{}", isoline_to_svg(&image, &vec![3.0]).unwrap());
        println!("{}", isoline_to_svg(&image, &vec![3.0,5.0,7.0]).unwrap());

        let marching_squares = MarchingSquares::new(&image);
        let IsolineLayer{paths, threshold:_} = marching_squares.isoline(5.0);

        for path in paths {
            println!("{:?}", path);
        }
    }

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_tiff_isoline() {
        let mut f: File = File::open("Seattle_Cropped.tif").unwrap();
        let mut buffer: Vec<u8> = Vec::new();

        f.read_to_end(&mut buffer).unwrap();

        let svg: Svg = isoline_from_tiff(&buffer, &vec![25.0, 50.0, 75.0, 100.0]);
        println!("{}", svg);
    }
}
