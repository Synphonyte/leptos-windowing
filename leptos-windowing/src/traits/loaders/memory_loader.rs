use std::ops::Range;

use crate::SortMode;

/// Loader trait for loading items on-demand from an in-memory data source.
///
/// In this case we don't need async methods and everything is simple and synchronous.
pub trait MemoryLoader {
    /// The type of items that will be loaded.
    type Item;

    /// Loads items from the given range, sorted according to the given sorting criteria.
    fn load_items(&self, range: Range<usize>, sorting: &[(usize, SortMode)]) -> Vec<Self::Item>;

    /// The total number of items of this data source.
    fn item_count(&self) -> usize;
}
