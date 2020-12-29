use wasm_bindgen::prelude::*;
mod marching_squares;
mod quad_tree;
mod util;

#[wasm_bindgen]
pub fn isoline() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
