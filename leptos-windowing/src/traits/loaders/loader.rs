use std::{fmt::Debug, ops::Range};

/// Loader trait for loading items on-demand from a data source.
///
/// This is the most generic loader trait. Please have a look first at the other loader traits as they
/// usually are simpler to implement. Only if they are not sufficient, you should implement this trait.
pub trait Loader {
    /// If this Some(...) then the data will be loaded in chunks of this size.
    /// This is useful for paginated data sources.
    /// Please look at [`PaginatedLoader`] if you have such a paginated data source.
    const CHUNK_SIZE: Option<usize> = None;

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
    /// It returns [`LoadedItems`] containing the loaded items as well with the loaded range
    /// which can be different from the requested range if the data source can't provide the
    /// exact requested range.
    fn load_items(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> impl Future<Output = Result<LoadedItems<Self::Item>, Self::Error>>;

    /// The total number of items of this data source with respect to the query.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn item_count(&self, _query: &Self::Query) -> impl Future<Output = Result<Option<usize>, Self::Error>> {
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
