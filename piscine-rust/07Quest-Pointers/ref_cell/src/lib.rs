pub mod messenger;

// This line makes `Tracker` available at the root of your crate. 
// So when `main.rs` says `use ref_cell::*;`, it will successfully find `Tracker`.
pub use messenger::Tracker;