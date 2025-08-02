use std::ops::Range;

/// Loader trait for loading items on-demand from an in-memory data source.
///
/// In this case we don't need async methods and everything is simple and synchronous.
pub trait MemoryLoader {
    /// The type of items that will be loaded.
    type Item;

    /// The type of the query data that will be used to load items.
    ///
    /// Can be used to filter or sort the items for example.
    type Query;

    /// Loads items from the given range, respecting the query.
    fn load_items(&self, range: Range<usize>, query: &Self::Query) -> Vec<Self::Item>;

    /// The total number of items of this data source with respect to the query.
    fn item_count(&self, query: &Self::Query) -> usize;
}
