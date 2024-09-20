use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::to_value;
use serde::Serialize;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct TreeInfo {
    pub x: f64,
    pub y: f64,
    pub species: u8,
}

#[wasm_bindgen]
pub struct SharedBuffer {
    ptr: *mut f64,
    len: usize,
}

#[wasm_bindgen]
impl SharedBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(num_trees: usize) -> SharedBuffer {
        // Each tree will have 3 values: x, y (f64), and species (u8 stored as f64)
        let size = num_trees * 3;
        let buffer = vec![0f64; size].into_boxed_slice(); // Allocate memory
        let ptr = buffer.as_ptr() as *mut f64; // Get raw pointer to the buffer
        let len = buffer.len(); // Length of the buffer
        std::mem::forget(buffer); // Prevent Rust from freeing this memory
        SharedBuffer { ptr, len }
    }

    pub fn ptr(&self) -> *mut f64 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Fills the buffer with data for a single tree (x, y, species)
    /// `index` is the index of the tree in the buffer (0-based)
    pub fn fill_tree(&self, index: usize, x: f64, y: f64, species: u8) {
        let base = index * 3; // 3 values per tree: x, y, species
        if base + 2 < self.len {
            unsafe {
                *self.ptr.add(base) = x;           // x coordinate
                *self.ptr.add(base + 1) = y;       // y coordinate
                *self.ptr.add(base + 2) = species as f64; // species as u8 stored in f64
            }
        }
    }

    pub fn read_trees(&self) -> JsValue {
        let mut trees = Vec::new();
        let count = self.len / 3; // Number of trees
    
        for i in 0..count {
            let base = i * 3;
            let x = unsafe { *self.ptr.add(base) };
            let y = unsafe { *self.ptr.add(base + 1) };
            let species = unsafe { *self.ptr.add(base + 2) } as u8;

            // Create a Tree struct instance
            let tree = TreeInfo { x, y, species };
    
            trees.push(tree);
        }
    
        // Convert the vector of trees into a JsValue
        to_value(&trees).unwrap() // Now this should work
    }
}

impl Drop for SharedBuffer {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.ptr, self.len, self.len);
        }
    }
}

