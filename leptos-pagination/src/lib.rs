//! Pagination for Leptos.
//!
//! This crate contains hooks and components for easy pagination of data.
//! It provides efficient loading, caching and displaying of large data. At the same time it
//! is very easy to use even for small datasets.
//!
//! ## Loading data
//!
//! Loading data is done through implementing one of the various `Loader` traits. Depending on your use case
//! you should implement the trait that best fits your needs:
//!
//! - [`MemoryLoader`]: If your dataset is already in memory like in a `Vec`, `HashSet`, array, ...
//! - [`PaginatedLoader`]: If your data source provides data in pages (independent of if you use UI pagination or virtualization).
//! - [`ExactLoader`]: If your data source can provide an exact range of items (start index to end index).
//! - [`Loader`]: If none of the above fit your needs, you can implement this trait to provide your own loading logic.
//!
//! Please refer to the documentation and the examples to see how to implement these traits.
//!
//! ## Components
//!
//! This crate provides several components designed to help you with the pagination of data.
//! These components are:
//!
//! - [`PaginatedFor`]: A component that displays a list of items in a paginated manner.
//! - [`PaginationPages`]: A component that displays the buttons to jump to a certain page.
//! - [`PaginationNext`]: A component that displays a button to navigate to the next page.
//! - [`PaginationPrev`]: A component that displays a button to navigate to the previous page.
//!
//! Please refer to the examples to see how to use these components.
//!
//! ## Hooks
//!
//! All components are just thin wrappers that add commonly used html to hook functions that implement the actual logic.
//! So if you want to customize your markup more than what the pre-made components allow you can use these hooks directly.
//!
//! These are the hooks:
//!
//! - [`use_pagination`]: Logic for [`PaginatedFor`]. Handles loading items on-demand from the data source and caching them.
//! - [`use_pagination_controls`]: Logic for [`PaginationPages`]. Returns page ranges that can be used to display pagination controls.
//!
//! If you want to implement your own custom components using these hooks, please have a look at the pre-made components in this crate.
//! You'll see that there is really nothing special about them.

mod components;
mod hooks;
mod state;

pub use components::*;
pub use hooks::*;
pub use state::*;

pub use leptos_windowing::*;
