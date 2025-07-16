use std::{fmt::Debug, ops::Range};

use crate::SortMode;

/// Trait for loading items on-demand from an data source that let's you request precise ranges.
///
/// Implement this if your data source actually returns exactly the range of items requested and
/// if it can provide the total number of items.
pub trait ExactLoader {
    /// The type of items that will be loaded.
    type Item;

    /// The type of errors that can occur during loading.
    type Error: Debug;

    fn load_items(
        &self,
        range: Range<usize>,
        sorting: &[(usize, SortMode)],
    ) -> impl Future<Output = Result<Vec<Self::Item>, Self::Error>>;

    /// The total number of items of this data source.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn item_count(&self) -> impl Future<Output = Result<usize, Self::Error>>;
}
