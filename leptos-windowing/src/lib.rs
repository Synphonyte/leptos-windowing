//! Virtualization and pagination for Leptos.
//!
//! This crate contains hooks and components for easy virtualization and pagination of data.
//! It provides efficient loading, caching and displaying of large data. At the same time it
//! is very easy to use even for small datasets.

mod cache;
pub mod hook;
pub mod item_state;
pub mod pagination;
mod traits;
mod window;

pub use traits::*;
pub use window::*;
