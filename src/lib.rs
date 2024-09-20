pub mod forest_property;
pub mod geometry_utils;
pub mod geojson_utils;
pub mod jittered_hexagonal_sampling;
pub mod projection;
pub mod main_functions;
pub mod shared_buffer;

#[cfg(not(target_arch = "wasm32"))]
pub mod requests;

#[cfg(target_arch = "wasm32")]
pub mod requests_wasm;
