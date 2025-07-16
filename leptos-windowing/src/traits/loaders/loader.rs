use std::{fmt::Debug, ops::Range};

use crate::SortMode;

/// Loader trait for loading items on-demand from a data source.
///
/// This is the most generic loader trait. Please have a look first at the other loader traits as they
/// usually are simpler to implement. Only if they are not sufficient, you should implement this trait.
pub trait Loader {
    /// If this Some(...) then the data will be loaded in chunks of this size.
    /// This is useful for paginated data sources.
    const CHUNK_SIZE: Option<usize> = None;

    /// The type of items that will be loaded.
    type Item;

    /// The type of errors that can occur during loading.
    type Error: Debug;

    fn load_items(
        &self,
        range: Range<usize>,
        sorting: &[(usize, SortMode)],
    ) -> impl Future<Output = Result<LoadedItems<Self::Item>, Self::Error>>;

    /// The total number of items of this data source.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn item_count(&self) -> impl Future<Output = Result<Option<usize>, Self::Error>> {
        async { Ok(None) }
    }
}

/// Return type of [`Loader::load_items`].
pub struct LoadedItems<T> {
    /// The loaded items.
    pub items: Vec<T>,

    /// The actual range of items that were loaded.
    ///
    /// This may be different from the requested range, for example if the data source is paginated.
    pub range: Range<usize>,
}
