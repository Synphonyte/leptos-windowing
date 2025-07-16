use std::{fmt::Debug, ops::Range};

/// Trait for loading items on-demand from an data source that let's you request precise ranges.
///
/// Implement this if your data source actually returns exactly the range of items requested and
/// if it can provide the total number of items.
pub trait ExactLoader {
    /// The type of items that will be loaded.
    type Item;

    /// The type of the query data that will be used to load items.
    ///
    /// Can be used to filter or sort the items for example.
    type Query;

    /// The type of errors that can occur during loading.
    type Error: Debug;

    /// Does the actual loading of items.
    ///
    /// This will be called with a range respecting the chunk size.
    /// The query data can be used to filter or sort the items for example.
    ///
    /// It returns a list of items. If the number of items is less than the requested range,
    /// it means that the end of the data source has been reached.
    fn load_items(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> impl Future<Output = Result<Vec<Self::Item>, Self::Error>>;

    /// The total number of items of this data source with respect to the query.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn item_count(&self, _query: &Self::Query) -> impl Future<Output = Result<Option<usize>, Self::Error>> {
        async move { Ok(None) }
    }
}
