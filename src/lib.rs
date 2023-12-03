use kiddo::float::kdtree::KdTree;

use wasm_bindgen::prelude::*;

// Wasm has a 4GB memory limit. Should make sure the bucket size and capacity
// doesn't exceed it and cause stack overflow.
// More detail: https://v8.dev/blog/4gb-wasm-memory
const BUCKET_SIZE: usize = 32;

// pub type Tree = ImmutableKdTree<f32, u64, 384, 32>;
pub type Tree = KdTree<f32, u64, 384, BUCKET_SIZE, u16>;

#[wasm_bindgen]
pub struct Init {}

#[wasm_bindgen]
impl Init {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Init {
        let _tree: Tree = KdTree::new();
        Init {}
    }
}
