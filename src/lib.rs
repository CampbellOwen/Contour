use wasm_bindgen::prelude::*;
mod quad_tree;

#[wasm_bindgen]
pub fn test(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
